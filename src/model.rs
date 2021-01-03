use chrono::NaiveDateTime;
use crate::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct Todo {
    pub id: Option<i32>,
    pub name: String,
    pub completed: i32,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}