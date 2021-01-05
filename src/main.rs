use std::{
    fs::File,
    io::{BufReader, Read},
};

use actix_cors::Cors;
use actix_files::Files;
use actix_web::{http::ContentEncoding, middleware::Compress, App, HttpServer};

pub mod handler;
pub mod model;
pub mod schema;

#[macro_use]
extern crate diesel;

#[macro_use]
extern crate diesel_migrations;

use diesel::{
    r2d2::{ConnectionManager, Pool},
    SqliteConnection,
};

use handler::*;
use serde_json::Value;

embed_migrations!();

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = ConnectionManager::<SqliteConnection>::new("argems.sqlite3");
    let pool = Pool::builder()
        .max_size(1)
        .build(manager)
        .expect("Failed to create pool.");

    let mut SERVER_PORT = String::new();

    match File::open("env.json") {
        Ok(file) => {
            let mut file_contents = String::new();
            BufReader::new(file).read_to_string(&mut file_contents);

            match serde_json::from_str(&file_contents) as Result<Value, _> {
                Ok(val) => {
                    SERVER_PORT = match val["server_port"].as_str() {
                        Some(port) => port.to_string(),
                        _ => SERVER_PORT
                    };
                }
                _ => {
                    println!("Error parsing env.json");
                }
            }
        }
        _ => {
            panic!("Error opening env.json");
        }
    }
    println!("Running on http://localhost:{}!", SERVER_PORT);
    println!("{}", SERVER_PORT);

    // Run embedded migrations
    match pool.get() {
        Ok(conn) => {
            embedded_migrations::run(&conn);
        }
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
    .bind(format!("127.0.0.1:{}", SERVER_PORT))?
    .run()
    .await
}
