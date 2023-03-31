use std::{collections::BTreeMap, sync::Arc};

use crate::NewTrait;

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

impl NewTrait for Repo {}
