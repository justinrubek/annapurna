use serde::{Deserialize, Serialize};
use sqlx_ulid::Ulid;

pub mod chores;
pub mod entity;
pub mod error;
pub mod inventory;
pub mod people;

#[derive(Debug, Serialize, Deserialize)]
pub struct Pagination {
    pub last_key: Option<Ulid>,
    pub count: usize,
}

#[derive(Debug)]
pub struct User {
    pub user_id: Option<Ulid>,
    // pub user_id: Option<sqlx::types::Uuid>,
    pub identifier: String,
}
