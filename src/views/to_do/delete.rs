use actix_web::{web, HttpResponse};
use diesel::prelude::*;

use crate::database::DB;
use crate::diesel;
use crate::json_serialization::to_do_item::ToDoItem;
use crate::json_serialization::to_do_items::ToDoItems;
use crate::jwt::JwToken;
use crate::models::item::item::Item;
use crate::schema::to_do;

#[allow(dead_code)]
pub async fn delete(to_do_item: web::Json<ToDoItem>, token: JwToken, mut db: DB) -> HttpResponse {
    let items = to_do::table
        .filter(to_do::columns::title.eq(&to_do_item.title.as_str()))
        .filter(to_do::columns::user_id.eq(&token.user_id))
        .order(to_do::columns::id.asc())
        .load::<Item>(&mut db.connection)
        .unwrap();
    let _ = diesel::delete(&items[0]).execute(&mut db.connection);
    HttpResponse::Ok().json(ToDoItems::get_state(token.user_id))
}
