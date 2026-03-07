use crate::entities::item::Item;
use crate::entities::item_history::ItemHistory;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

const SECONDS_IN_HOUR: f64 = 3600.0;

#[derive(Debug, Serialize, Deserialize)]
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
            lead_time_hours,
        );

        Some(ItemMetrics {
            lead_time_hours,
            cycle_time_hours,
        })
    }

    fn calculate_lead_time(item: &Item, done_timestamp: &DateTime<Utc>) -> f64 {
        let seconds = done_timestamp
            .signed_duration_since(item.created_at)
            .num_seconds();

        seconds as f64 / SECONDS_IN_HOUR
    }

    fn calculate_cycle_time(
        histories: &[ItemHistory],
        wip_column_ids: &[Uuid],
        fall_back_lead_time: f64,
    ) -> f64 {
        let mut total_wip_seconds = 0;
        let mut wip_start: Option<DateTime<Utc>> = None;

        for history in histories {
            if wip_column_ids.contains(&history.new_column_id) {
                wip_start.get_or_insert(history.timestamp);
            } else if let Some(start) = wip_start.take() {
                    total_wip_seconds += history
                        .timestamp
                        .signed_duration_since(start)
                        .num_seconds();
                }
        }

        if let Some(start) = wip_start {
            total_wip_seconds += Utc::now().signed_duration_since(start).num_seconds();
        }

        if total_wip_seconds == 0 {
            return fall_back_lead_time;
        }

        total_wip_seconds as f64 / SECONDS_IN_HOUR
    }
}