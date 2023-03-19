#[macro_use]
extern crate diesel;
extern crate dotenv;

use actix_service::Service;
use actix_web::{middleware::Logger, App, HttpResponse, HttpServer};
use futures::future::{ok, Either};

mod database;
mod json_serialization;
mod jwt;
mod models;
mod schema;

mod config;
mod counter;
mod to_do;
mod views;

const ALLOWED_VERSION: &str = include_str!("./output_data.txt");

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    let site_counter = counter::Counter { count: 0 };
    site_counter
        .save()
        .expect("Failed to save counter value to key-value store. Initial value.");

    HttpServer::new(|| {
        App::new()
            .wrap_fn(|req, srv| {
                println!("{:?}", req);
                let passed = req.path().contains(&format!("/{}/", ALLOWED_VERSION));
                let mut site_counter = counter::Counter::load().unwrap();
                site_counter.count += 1;
                println!("{:?}", &site_counter);
                site_counter
                    .save()
                    .expect("Failed to save counter value to key-value store");
                let end_result = if passed {
                    Either::Left(srv.call(req))
                } else {
                    let resp = HttpResponse::NotImplemented()
                        .body(format!("only {} API is supported", ALLOWED_VERSION));
                    Either::Right(ok(req.into_response(resp).map_into_boxed_body()))
                };
                async {
                    let result = end_result.await?;
                    Ok(result)
                }
            })
            .configure(views::views_factory)
            .wrap(Logger::new("%a %{User-Agent}i %r %s %D"))
    })
    .bind("0.0.0.0:8000")?
    .run()
    .await
}
