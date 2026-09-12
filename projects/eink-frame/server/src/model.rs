use chrono::{DateTime, NaiveDate};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CalendarEvent {
    pub start: String,
    pub title: String,
    pub who: String,
    pub all_day: bool,
    pub day_label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TodoItem {
    pub title: String,
    pub done: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ShoppingItem {
    pub name: String,
    pub qty: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Dashboard {
    pub family_name: String,
    pub weekday: String,
    pub date_long: String,
    pub date_iso: String,
    pub events_today: Vec<CalendarEvent>,
    pub events_week: Vec<CalendarEvent>,
    pub todos: Vec<TodoItem>,
    pub shopping: Vec<ShoppingItem>,
    pub source_note: String,
}

impl Dashboard {
    pub fn empty(family_name: &str, date: NaiveDate) -> Self {
        Self {
            family_name: family_name.to_string(),
            weekday: date.format("%A").to_string(),
            date_long: date.format("%-d %B %Y").to_string(),
            date_iso: date.format("%Y-%m-%d").to_string(),
            events_today: Vec::new(),
            events_week: Vec::new(),
            todos: Vec::new(),
            shopping: Vec::new(),
            source_note: String::new(),
        }
    }

    /// Canonical payload hashed so an unchanged family day skips Chromium
    /// and the Pico can skip the panel refresh.
    pub fn content_bytes(&self) -> Vec<u8> {
        serde_json::to_vec(self).expect("dashboard json")
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct FrameInfo {
    pub checksum: String,
    pub bytes: usize,
    pub generated_at: DateTime<chrono::Utc>,
    pub content_hash: String,
    pub source_note: String,
}

#[derive(Debug, Deserialize)]
pub struct FileTodo {
    pub title: String,
    #[serde(default)]
    pub done: bool,
}

#[derive(Debug, Deserialize)]
pub struct FileShopping {
    pub name: String,
    #[serde(default)]
    pub qty: String,
}
