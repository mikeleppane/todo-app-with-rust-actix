use chrono::{NaiveDateTime, Utc};

use crate::schema::to_do;

#[derive(Insertable)]
#[diesel(table_name = to_do)]
pub struct NewItem {
    pub title: String,
    pub status: String,
    pub date: NaiveDateTime,
    pub user_id: i32,
}

impl NewItem {
    pub fn new(title: String, user_id: i32) -> Self {
        let now = Utc::now().naive_local();
        Self {
            title,
            status: String::from("PENDING"),
            date: now,
            user_id,
        }
    }
}
