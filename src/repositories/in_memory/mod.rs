use std::{collections::BTreeMap, sync::Arc};

pub mod player;
pub mod team;

pub struct Repo {
    pub pool: Arc<BTreeMap<String, String>>,
}

impl Repo {
    pub fn new() -> Self {
        let pool = Arc::new(BTreeMap::new());

        Self { pool }
    }
}
