use crate::entities::player::Player;
use crate::{services::queries::RepoTrait, Deps};
use std::sync::Arc;

struct ExecutorImpl<C> {
    deps: Arc<Deps<C>>,
}

pub fn new_executor<C: RepoTrait + Send + Sync + 'static>(deps: Arc<Deps<C>>) -> Box<dyn Executor> {
    Box::new(ExecutorImpl { deps })
}

#[async_trait::async_trait]
pub trait Executor: Send + Sync {
    async fn execute(&self, id: &str) -> Result<Option<Player>, String>;
}

#[async_trait::async_trait]
impl<C: RepoTrait + Send + Sync> Executor for ExecutorImpl<C> {
    async fn execute(&self, id: &str) -> Result<Option<Player>, String> {
        let res = self.deps.queries_repo.player_by_id(id).await?;

        Ok(res)
    }
}
