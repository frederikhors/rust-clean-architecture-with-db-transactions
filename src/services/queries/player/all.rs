use crate::entities::player::Player;
use crate::queries::player::PlayerAllInput;
use crate::{services::queries::RepoTrait, Deps};
use std::sync::Arc;

struct ExecutorImpl<C: ?Sized> {
    deps: Arc<Deps<C>>,
}

pub fn new_executor<C: RepoTrait + Send + Sync + 'static + ?Sized>(deps: Arc<Deps<C>>) -> Box<dyn Executor> {
    Box::new(ExecutorImpl { deps })
}

#[async_trait::async_trait]
pub trait Executor: Send + Sync {
    async fn execute(&self, input: &PlayerAllInput) -> Result<Vec<Player>, String>;
}

#[async_trait::async_trait]
impl<C: RepoTrait + Send + Sync + ?Sized> Executor for ExecutorImpl<C> {
    async fn execute(&self, input: &PlayerAllInput) -> Result<Vec<Player>, String> {
        let all = self.deps.queries_repo.player_all(input).await?;

        Ok(all)
    }
}
