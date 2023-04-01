use crate::entities::player::Player;
use crate::queries::player::PlayerAllInput;
use crate::queries::RepoPlayer;
use crate::Deps;
use std::sync::Arc;

struct ExecutorImpl<C, Q> {
    deps: Arc<Deps<C, Q>>,
}

pub fn new_executor<C: Send + Sync + 'static, Q: Send + Sync + RepoPlayer + 'static>(
    deps: Arc<Deps<C, Q>>,
) -> Box<dyn Executor> {
    Box::new(ExecutorImpl { deps })
}

#[async_trait::async_trait]
pub trait Executor: Send + Sync {
    async fn execute(&self, input: &PlayerAllInput) -> Result<Vec<Player>, String>;
}

#[async_trait::async_trait]
impl<C: Send + Sync, Q: Send + Sync + RepoPlayer> Executor for ExecutorImpl<C, Q> {
    async fn execute(&self, input: &PlayerAllInput) -> Result<Vec<Player>, String> {
        let all = self.deps.queries_repo.player_all(input).await?;

        Ok(all)
    }
}
