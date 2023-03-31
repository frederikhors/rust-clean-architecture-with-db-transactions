use std::sync::Arc;

pub mod player;
pub mod team;

pub struct Repo {
    pub pool: Arc<sqlx::PgPool>,
}

impl Repo {
    pub fn new(pool: Arc<sqlx::PgPool>) -> Self {
        Self { pool }
    }
}
