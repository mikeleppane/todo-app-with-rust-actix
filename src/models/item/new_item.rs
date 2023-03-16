use chrono::{NaiveDateTime, Utc};

use crate::schema::to_do;

#[derive(Insertable)]
#[table_name = "to_do"]
pub struct NewItem {
    pub title: String,
    pub status: String,
    pub date: NaiveDateTime,
}

impl NewItem {
    pub fn new(title: String) -> Self {
        let now = Utc::now().naive_local();
        Self {
            title,
            status: String::from("PENDING"),
            date: now,
        }
    }
}
