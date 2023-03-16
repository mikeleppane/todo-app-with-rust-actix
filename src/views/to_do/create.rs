use actix_web::{HttpRequest, HttpResponse};
use diesel::prelude::*;

use crate::database::DB;
use crate::diesel;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::models::item::item::Item;
use crate::models::item::new_item::NewItem;
use crate::schema::to_do;

pub async fn create(req: HttpRequest, mut db: DB) -> HttpResponse {
    let title: String = req.match_info().get("title").unwrap().to_string();
    let items = to_do::table
        .filter(to_do::columns::title.eq(&title.as_str()))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut db.connection)
        .unwrap();
    if items.is_empty() {
        let new_post = NewItem::new(title, 1);
        let _ = diesel::insert_into(to_do::table)
            .values(&new_post)
            .execute(&mut db.connection);
    }
    HttpResponse::Ok().json(ToDoItems::get_state())
}
