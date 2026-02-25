use crate::entities::item::Item;
use crate::entities::item_history::ItemHistory;
use chrono::{DateTime, Utc};
use uuid::Uuid;

const HOUR_IN_MINUTES: f64 = 60.0;

#[derive(Debug)]
pub struct ItemMetrics {
    pub lead_time_hours: f64,
    pub cycle_time_hours: f64,
}

pub struct ItemMetricsCalculator;

impl ItemMetricsCalculator {
    pub fn calculate(
        item: &Item,
        histories: &mut [ItemHistory],
        wip_column_ids: &[Uuid],
        done_column_ids: &[Uuid],
    ) -> Option<ItemMetrics> {
        if !item.is_done {
            return None;
        }

        histories.sort_by_key(|h| h.timestamp);
        let done_timestamp = histories
            .iter()
            .rev()
            .find(|h| done_column_ids.contains(&h.new_column_id))
            .map(|h| h.timestamp)?;

        let lead_time_hours = Self::calculate_lead_time(item, &done_timestamp);
        let cycle_time_hours = Self::calculate_cycle_time(
            histories,
            wip_column_ids,
            &done_timestamp,
            lead_time_hours,
        );

        Some(ItemMetrics {
            lead_time_hours,
            cycle_time_hours,
        })
    }

    fn calculate_lead_time(item: &Item, done_timestamp: &DateTime<Utc>) -> f64 {
        done_timestamp
            .signed_duration_since(item.created_at)
            .num_minutes() as f64
            / HOUR_IN_MINUTES
    }

    fn calculate_cycle_time(
        histories: &[ItemHistory],
        wip_column_ids: &[Uuid],
        done_timestamp: &DateTime<Utc>,
        fall_back_lead_time: f64,
    ) -> f64 {
        let started_at = histories
            .iter()
            .find(|h| wip_column_ids.contains(&h.new_column_id))
            .map(|h| h.timestamp);

        if let Some(started_at) = started_at {
            return done_timestamp
                .signed_duration_since(started_at)
                .num_minutes() as f64
                / HOUR_IN_MINUTES;
        }

        fall_back_lead_time
    }
}
