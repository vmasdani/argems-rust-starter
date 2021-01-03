use std::path::PathBuf;

use actix_web::{HttpRequest, HttpResponse, Responder, delete, error::BlockingError, get, post, web, web::Path};
use diesel::{
    r2d2::{self, ConnectionManager},
    SqliteConnection,
};

type DbPool = r2d2::Pool<ConnectionManager<SqliteConnection>>;

use crate::{model::Todo, schema};
use diesel::prelude::*;

#[get("/todos")]
pub async fn all_todos(pool: web::Data<DbPool>) -> impl Responder {
    match pool.get() {
        Ok(conn) => {
            use schema::todos::dsl::*;

            let todos_res = web::block(move || todos.load::<Todo>(&conn)).await;

            match todos_res {
                Ok(todos_all) => HttpResponse::Ok().json(todos_all),
                _ => HttpResponse::InternalServerError().body("Error"),
            }
        }
        _ => HttpResponse::InternalServerError().body("Pool connection errro."),
    }
}

#[get("/todos/{todo_id}")]
pub async fn get_todo(pool: web::Data<DbPool>, todo_id: Path<i32>) -> impl Responder {
    match pool.get() {
        Ok(conn) => {
            use schema::todos::dsl::*;

            let todos_res = web::block(move || {
                todos
                    .filter(id.eq(todo_id.into_inner()))
                    .first::<Todo>(&conn)
            })
            .await;

            match todos_res {
                Ok(todo) => HttpResponse::Ok().json(todo),
                _ => HttpResponse::InternalServerError().body("Error"),
            }
        }
        _ => HttpResponse::InternalServerError().body("Pool connection errro."),
    }
}

#[delete("/todos/{todo_id}")]
pub async fn delete_todo(pool: web::Data<DbPool>, todo_id: Path<i32>) -> impl Responder {
    match pool.get() {
        Ok(conn) => {
            use schema::todos::dsl::*;

            let todos_res = web::block(move || {
                diesel::delete(todos.filter(id.eq(todo_id.into_inner()))).execute(&conn)
            })
            .await;

            match todos_res {
                Ok(_) => HttpResponse::Ok().json("OK"),
                _ => HttpResponse::InternalServerError().body("Error"),
            }
        }
        _ => HttpResponse::InternalServerError().body("Pool connection errro."),
    }
}

#[post("/todos")]
pub async fn post_todo(pool: web::Data<DbPool>, todo: web::Json<Todo>) -> impl Responder {
    match pool.get() {
        Ok(conn) => {
            use schema::todos::dsl::*;

            let todos_res = web::block(move || {
                diesel::replace_into(todos)
                    .values(&todo.into_inner())
                    .execute(&conn);
                todos.order_by(id.desc()).first::<Todo>(&conn)
            })
            .await;

            match todos_res {
                Ok(todo) => HttpResponse::Ok().json(todo),
                _ => HttpResponse::InternalServerError().body("Error"),
            }
        }
        _ => HttpResponse::InternalServerError().body("Pool connection errro."),
    }
}

// #[get("/{filename:.*}")]
// pub async fn home(req: HttpRequest) -> impl Responder {
//     let path: PathBuf = req.match_info().query("filename").parse().unwrap();

//     HttpResponse::Ok().body(NamedFile::open(path))
// }
