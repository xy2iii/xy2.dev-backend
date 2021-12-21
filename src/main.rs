mod db;
mod resources;

use crate::db::init_pool;
use crate::resources::Comment;
// use crate::resources::{Comment, Reaction};
use actix_cors::Cors;
use actix_ratelimit::MemoryStore;
use actix_web::middleware::{Logger, NormalizePath, TrailingSlash};
use actix_web::{middleware, web, App, HttpServer};
use env_logger::Env;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let pool = init_pool().await.expect("Could not create pool");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(Logger::default())
            .wrap(Cors::permissive())
            .wrap(NormalizePath::new(TrailingSlash::Trim))
            .configure(Comment::configure)
        // .configure(Reaction::configure)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
