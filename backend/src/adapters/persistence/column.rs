use crate::adapters::persistence::PostgresPersistence;
use crate::entities::board_column::BoardColumn;
use crate::entities::column_type::ColumnType;
use crate::prelude::*;
use crate::use_cases::column::ColumnPersistence;
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(sqlx::Type, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[sqlx(type_name = "column_kind", rename_all = "lowercase")]
pub enum ColumnKindDb {
    Todo,
    Wip,
    Done,
}

#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct ColumnDb {
    pub id: Uuid,
    pub name: String,
    pub order_index: i64,
    pub column_type: ColumnKindDb,
    pub column_limit: Option<i32>,
    pub board_id: Uuid,
}

impl From<ColumnType> for ColumnKindDb {
    fn from(value: ColumnType) -> Self {
        match value {
            ColumnType::Todo { .. } => ColumnKindDb::Todo,
            ColumnType::Wip { .. } => ColumnKindDb::Wip,
            ColumnType::Done => ColumnKindDb::Done,
        }
    }
}

impl From<ColumnDb> for BoardColumn {
    fn from(value: ColumnDb) -> Self {
        let kind = match value.column_type {
            ColumnKindDb::Todo => ColumnType::Todo {
                limit: value.column_limit.map(|l| l as usize),
            },
            ColumnKindDb::Wip => ColumnType::Wip {
                limit: value.column_limit.map(|l| l as usize),
            },
            ColumnKindDb::Done => ColumnType::Done,
        };

        Self {
            id: value.id,
            board_id: value.board_id,
            name: value.name,
            kind,
            order_index: value.order_index as usize,
        }
    }
}

pub fn extract_db_kind_and_limit(value: &ColumnType) -> (ColumnKindDb, Option<i32>) {
    match value {
        ColumnType::Todo { limit: l } => (ColumnKindDb::Todo, l.map(|value| value as i32)),
        ColumnType::Wip { limit: l } => (ColumnKindDb::Wip, l.map(|value| value as i32)),
        ColumnType::Done => (ColumnKindDb::Done, None),
    }
}
#[async_trait]
impl ColumnPersistence for PostgresPersistence {
    async fn get_item_count(&self, column_id: Uuid) -> Result<usize> {
        let result = sqlx::query_scalar!(
            r#"SELECT COUNT(*) FROM board_items
            WHERE column_id = $1"#,
            column_id
        )
            .fetch_one(&self.pool)
            .await?
            .unwrap_or(0);

        Ok(result as usize)
    }

    async fn create_column(&self, column: &BoardColumn) -> Result<Uuid> {
        let (kind, limit) = extract_db_kind_and_limit(&column.kind);
        let id = sqlx::query_scalar!(r#"INSERT INTO board_columns (id, name, column_type, column_limit, order_index, board_id)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id"#,
        column.id,
        column.name,
        kind as ColumnKindDb,
        limit,
        column.order_index as i64,
        column.board_id
        )
            .fetch_one(&self.pool)
            .await?;

        Ok(id)
    }

    async fn save_all(&self, columns: &[BoardColumn]) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        for column in columns {
            let (kind, limit) = extract_db_kind_and_limit(&column.kind);
            sqlx::query!(r#"INSERT INTO board_columns (id, name, column_type, column_limit, order_index, board_id)
            VALUES ($1, $2, $3, $4, $5, $6)
            ON CONFLICT (id) DO UPDATE
            SET name = EXCLUDED.name,
            column_type = EXCLUDED.column_type,
            column_limit = EXCLUDED.column_limit,
            order_index = EXCLUDED.order_index"#,
        column.id,
        column.name,
        kind as ColumnKindDb,
        limit,
        column.order_index as i64,
        column.board_id
        )
                .execute(&mut *tx)
                .await?;
        }

        tx.commit().await?;

        Ok(())
    }

    async fn get_column(&self, id: Uuid) -> Result<Option<BoardColumn>> {
        let result = sqlx::query_as!(
            ColumnDb,
            r#"SELECT id, name, order_index,
            column_type AS "column_type: ColumnKindDb",
            column_limit,
            board_id
            FROM board_columns
            WHERE id = $1
            ORDER BY order_index ASC
            "#,
            id
        )
            .fetch_optional(&self.pool)
            .await?
            .map(Into::into);

        Ok(result)
    }

    async fn get_by_board_id(&self, board_id: Uuid) -> Result<Vec<BoardColumn>> {
        let result = sqlx::query_as!(
            ColumnDb,
            r#"SELECT id, name, order_index,
            column_type AS "column_type: ColumnKindDb",
            column_limit,
            board_id
            FROM board_columns
            WHERE board_id = $1
            ORDER BY order_index ASC
            "#,
            board_id
        )
            .fetch_all(&self.pool)
            .await?
            .into_iter()
            .map(Into::into)
            .collect();

        Ok(result)
    }

    async fn update_column(&self, column: &BoardColumn) -> Result<()> {
        let (kind, limit) = extract_db_kind_and_limit(&column.kind);

        sqlx::query!(
            r#"UPDATE board_columns SET name = $1, 
                         column_type = $2,
                         column_limit = $3, 
                         order_index = $4
                         WHERE id = $5"#,
            column.name,
            kind as ColumnKindDb,
            limit,
            column.order_index as i64,
            column.id
        )
            .execute(&self.pool)
            .await?;

        Ok(())
    }

    async fn delete_column(&self, id: Uuid) -> Result<()> {
        sqlx::query!(r#"DELETE from board_columns WHERE id = $1"#, id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
