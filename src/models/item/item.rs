use chrono::NaiveDateTime;

use crate::models::user::user::User;
use crate::schema::to_do;

#[derive(Queryable, Identifiable, Associations)]
#[belongs_to(User)]
#[table_name = "to_do"]
pub struct Item {
    pub id: i32,
    pub title: String,
    pub status: String,
    pub date: NaiveDateTime,
    pub user_id: i32,
}
