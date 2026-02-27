use crate::adapters::persistence::PostgresPersistence;
use crate::entities::item::Item;
use crate::entities::item_history::ItemHistory;
use crate::entities::item_priority::ItemPriority;
use crate::prelude::*;
use crate::use_cases::item::ItemPersistence;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{Postgres, Transaction};
use uuid::Uuid;
#[derive(sqlx::Type, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[sqlx(type_name = "item_priority", rename_all = "lowercase")]
pub enum ItemPriorityDb {
    Low,
    Medium,
    High,
}
#[derive(sqlx::FromRow, Debug, Serialize)]
pub struct ItemDb {
    pub id: Uuid,
    pub title: String,
    pub description: Option<String>,
    pub priority: ItemPriorityDb,
    pub done: bool,
    pub board_id: Uuid,
    pub column_id: Uuid,
    pub assigned_to: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

impl From<ItemPriorityDb> for ItemPriority {
    fn from(value: ItemPriorityDb) -> Self {
        match value {
            ItemPriorityDb::High => ItemPriority::High,
            ItemPriorityDb::Medium => ItemPriority::Medium,
            ItemPriorityDb::Low => ItemPriority::Low,
        }
    }
}

impl From<ItemPriority> for ItemPriorityDb {
    fn from(value: ItemPriority) -> Self {
        match value {
            ItemPriority::High => ItemPriorityDb::High,
            ItemPriority::Medium => ItemPriorityDb::Medium,
            ItemPriority::Low => ItemPriorityDb::Low,
        }
    }
}

impl From<ItemDb> for Item {
    fn from(value: ItemDb) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.description,
            is_done: value.done,
            priority: value.priority.into(),
            assigned_to: value.assigned_to,
            created_at: value.created_at,
            column_id: value.column_id,
            board_id: value.board_id,
        }
    }
}
pub struct ItemHistoryDb {
    id: Uuid,
    timestamp: DateTime<Utc>,
    prev_column_id: Option<Uuid>,
    new_column_id: Uuid,
    item_id: Uuid,
}

impl From<ItemHistoryDb> for ItemHistory {
    fn from(value: ItemHistoryDb) -> Self {
        Self {
            id: value.id,
            timestamp: value.timestamp,
            new_column_id: value.new_column_id,
            prev_column_id: value.prev_column_id,
            item_id: value.item_id,
        }
    }
}

#[async_trait]
impl ItemPersistence for PostgresPersistence {
    async fn get_items_by_column_with_limit_offset(
        &self,
        column_id: Uuid,
        limit: usize,
        offset: usize,
    ) -> Result<Vec<Item>> {
        let result = sqlx::query_as!(
            ItemDb,
            r#"
            SELECT
                id,
                title,
                description,
                priority AS "priority: ItemPriorityDb",
                done,
                board_id,
                column_id,
                assigned_to,
                created_at
            FROM board_items
            WHERE column_id = $1
            ORDER BY created_at
            LIMIT $2 OFFSET $3
            "#,
            column_id,
            limit as i64,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(Item::from)
        .collect();

        Ok(result)
    }

    async fn get_item_history(&self, item_id: Uuid) -> Result<Vec<ItemHistory>> {
        let result = sqlx::query_as!(
            ItemHistoryDb,
            r#"SELECT 
        id,
        new_column_id,
        prev_column_id,
        item_id,
        timestamp
        FROM item_histories
        WHERE item_id = $1
        ORDER BY timestamp ASC"#,
            item_id
        )
        .fetch_all(&self.pool)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();

        Ok(result)
    }

    async fn get_item(&self, id: Uuid) -> Result<Option<Item>> {
        let result = sqlx::query_as!(
            ItemDb,
            r#"
            SELECT
                id,
                title,
                description,
                priority AS "priority: ItemPriorityDb",
                done,
                board_id,
                column_id,
                assigned_to,
                created_at
            FROM board_items
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .map(Into::into);

        Ok(result)
    }

    async fn create_item(&self, item: &Item, history: &ItemHistory) -> Result<Uuid> {
        let mut tx = self.pool.begin().await?;

        let priority_db: ItemPriorityDb = ItemPriorityDb::from(item.priority.clone());

        let id = sqlx::query_scalar!(r#"INSERT INTO board_items (id, title, description, done, priority, assigned_to, created_at, column_id, board_id)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            RETURNING id
            "#,
            item.id,
            item.title,
            item.description,
            item.is_done,
            priority_db as ItemPriorityDb,
            item.assigned_to,
            item.created_at,
            item.column_id,
            item.board_id)
            .fetch_one(&mut *tx)
            .await?;

        Self::create_item_history(history, &mut tx).await?;

        tx.commit().await?;

        Ok(id)
    }

    async fn update_item(&self, item: &Item, history: Option<&ItemHistory>) -> Result<()> {
        let priority_db: ItemPriorityDb = ItemPriorityDb::from(item.priority.clone());

        let mut tx = self.pool.begin().await?;

        sqlx::query!(
            r#"UPDATE board_items SET title = $2,
                    description = $3,
                    priority = $4,
                    done = $5,
                    column_id = $6,
                    assigned_to = $7
                    WHERE id = $1"#,
            item.id,
            item.title,
            item.description,
            priority_db as ItemPriorityDb,
            item.is_done,
            item.column_id,
            item.assigned_to
        )
        .execute(&mut *tx)
        .await?;

        if let Some(history) = history {
            Self::create_item_history(history, &mut tx).await?;
        }

        tx.commit().await?;

        Ok(())
    }

    async fn delete_item(&self, id: Uuid) -> Result<()> {
        sqlx::query!(r#"DELETE from board_items WHERE id = $1"#, id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}

impl PostgresPersistence {
    async fn create_item_history(
        history: &ItemHistory,
        tx: &mut Transaction<'_, Postgres>,
    ) -> Result<()> {
        sqlx::query!(
            r#"INSERT INTO item_histories (id, timestamp, prev_column_id, new_column_id, item_id)
                        VALUES ($1, $2, $3, $4, $5)"#,
            history.id,
            history.timestamp,
            history.prev_column_id,
            history.new_column_id,
            history.item_id
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }
}
