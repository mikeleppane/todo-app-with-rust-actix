use actix_web::dev::Payload;
use actix_web::error::ErrorServiceUnavailable;
use actix_web::{Error, FromRequest, HttpRequest};
use diesel::r2d2::{ConnectionManager, Pool, PooledConnection};
use diesel::PgConnection;
use futures::future::{err, ok, Ready};
use lazy_static::lazy_static;

use crate::config::Config;

type PgPool = Pool<ConnectionManager<PgConnection>>;

pub struct DbConnection {
    pub db_connection: PgPool,
}

pub struct DB {
    pub connection: PooledConnection<ConnectionManager<PgConnection>>,
}

lazy_static! {
    pub static ref DBCONNECTION: DbConnection = {
        let connection_string = Config::new()
            .map
            .get("DB_URL")
            .unwrap()
            .as_str()
            .unwrap()
            .to_string();
        DbConnection {
            db_connection: PgPool::builder()
                .max_size(8)
                .build(ConnectionManager::new(connection_string))
                .expect("failed to create db connection_pool"),
        }
    };
}

pub fn establish_connection() -> PooledConnection<ConnectionManager<PgConnection>> {
    DBCONNECTION.db_connection.get().unwrap()
}

impl FromRequest for DB {
    type Error = Error;
    type Future = Ready<Result<DB, Error>>;
    fn from_request(_: &HttpRequest, _: &mut Payload) -> Self::Future {
        match DBCONNECTION.db_connection.get() {
            Ok(connection) => ok(DB { connection }),
            Err(_) => err(ErrorServiceUnavailable(
                "could not make connection to database",
            )),
        }
    }
}
