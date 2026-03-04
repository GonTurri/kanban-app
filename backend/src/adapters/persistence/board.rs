use crate::adapters::persistence::PostgresPersistence;
use crate::adapters::persistence::column::{ColumnKindDb, extract_db_kind_and_limit};
use crate::entities::board::Board;
use crate::entities::board_column::BoardColumn;
use crate::entities::board_member::BoardMember;
use crate::entities::board_role::BoardRole;
use crate::prelude::*;
use crate::use_cases::board::BoardPersistence;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct BoardDb {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub title: String,
    pub description: String,
    pub members: Json<Vec<BoardMemberDb>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BoardMemberDb {
    pub id: Uuid,
    pub user_id: Uuid,
    pub role: BoardRoleDb,
}

#[derive(sqlx::Type, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[sqlx(type_name = "board_role", rename_all = "lowercase")]
#[serde(rename_all = "lowercase")]
pub enum BoardRoleDb {
    Owner,
    Editor,
    Viewer,
}

impl From<BoardDb> for Board {
    fn from(value: BoardDb) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
            owner_id: value.owner_id,
            members: value.members.0.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<BoardMemberDb> for BoardMember {
    fn from(value: BoardMemberDb) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            role: value.role.into(),
        }
    }
}

impl From<BoardRoleDb> for BoardRole {
    fn from(value: BoardRoleDb) -> Self {
        match value {
            BoardRoleDb::Owner => BoardRole::Owner,
            BoardRoleDb::Editor => BoardRole::Editor,
            BoardRoleDb::Viewer => BoardRole::Viewer,
        }
    }
}

impl From<BoardRole> for BoardRoleDb {
    fn from(value: BoardRole) -> Self {
        match value {
            BoardRole::Owner => BoardRoleDb::Owner,
            BoardRole::Editor => BoardRoleDb::Editor,
            BoardRole::Viewer => BoardRoleDb::Viewer,
        }
    }
}

#[async_trait]
impl BoardPersistence for PostgresPersistence {
    async fn create_board(&self, board: &Board, columns: &[BoardColumn]) -> Result<Uuid> {
        let mut tx = self.pool.begin().await?;

        let id = sqlx::query_scalar!(
            r#"INSERT INTO boards (id, title, description, owner_id)
       VALUES ($1, $2, $3, $4)
       RETURNING id"#,
            board.id,
            board.title,
            board.description,
            board.owner_id
        )
        .fetch_one(&mut *tx)
        .await?;

        for member in &board.members {
            let role_db: BoardRoleDb = member.role.clone().into();
            sqlx::query!(
                r#"INSERT INTO board_members (id, user_id, board_id, role) VALUES ($1, $2, $3, $4)"#,
                member.id, member.user_id, board.id, role_db as BoardRoleDb
            )
                .execute(&mut *tx)
                .await?;
        }

        for column in columns {
            let (kind, limit) = extract_db_kind_and_limit(&column.kind);

            sqlx::query!(r#"INSERT INTO board_columns (id, name, column_type, column_limit, order_index, board_id)
            VALUES ($1, $2, $3, $4, $5, $6)"#,column.id,
        column.name,
        kind as ColumnKindDb,
        limit,
        column.order_index as i64,
        column.board_id)
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;

        Ok(id)
    }

    async fn get_board(&self, id: Uuid) -> Result<Option<Board>> {
        let result = sqlx::query_as!(
            BoardDb,
            r#"
            SELECT
                b.id,
                b.owner_id,
                b.title,
                b.description,
                JSONB_AGG(
                    JSONB_BUILD_OBJECT(
                        'id', m.id,
                        'user_id', m.user_id,
                        'role', m.role
                    )
                ) AS "members!: Json<Vec<BoardMemberDb>>"
            FROM boards b
            INNER JOIN board_members m ON b.id = m.board_id
            WHERE b.id = $1
            GROUP BY b.id
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into);

        Ok(result)
    }

    async fn add_member_to_board(&self, board_id: Uuid, member: &BoardMember) -> Result<Uuid> {
        let role_db: BoardRoleDb = member.role.into();
        let id = sqlx::query_scalar!(
            r#"INSERT INTO board_members (id, user_id, board_id, role)
        VALUES ($1, $2, $3, $4)
        RETURNING id"#,
            member.id,
            member.user_id,
            board_id,
            role_db as BoardRoleDb
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(id)
    }

    async fn update_member(&self, member: &BoardMember) -> Result<()> {
        let role_db: BoardRoleDb = member.role.into();
        sqlx::query!(
            r#"UPDATE board_members SET role = $2 WHERE id = $1"#,
            member.id,
            role_db as BoardRoleDb
        )
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    async fn remove_member_from_board(&self, member_id: Uuid) -> Result<()> {
        sqlx::query!(r#"DELETE FROM board_members WHERE id = $1"#, member_id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn exists_by_id(&self, id: Uuid) -> Result<bool> {
        let result =
            sqlx::query_scalar!(r#"SELECT EXISTS(SELECT 1 FROM boards WHERE id = $1)"#, id)
                .fetch_one(&self.pool)
                .await?
                .unwrap_or(false);

        Ok(result)
    }
}
