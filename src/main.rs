use actix_cors::Cors;
use actix_files::Files;
use actix_web::{http::ContentEncoding, middleware::Compress, App, HttpServer};

pub mod handler;
pub mod schema;
pub mod model;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};

use handler::*;

embed_migrations!(); 

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = ConnectionManager::<SqliteConnection>::new("argems.sqlite3");
    let pool = Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Failed to create pool.");

    println!("Running on http://localhost:8080!");

    // Run embedded migrations
    match pool.get() {
        Ok(conn) => {
            embedded_migrations::run(&conn);
        },
        _ => {
            println!("Failed getting pool for migration");
        }
    }

    HttpServer::new(move || {
        let pool_clone = pool.clone();

        App::new()
            .data(pool_clone)
            .wrap(
                Cors::default()
                    .allow_any_header()
                    .allow_any_origin()
                    .allow_any_method(),
            )
            .wrap(Compress::new(ContentEncoding::Br))
            .service(all_todos)
            .service(get_todo)
            .service(delete_todo)
            .service(post_todo)
            .service(Files::new("/", "./frontend").index_file("index.html"))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
