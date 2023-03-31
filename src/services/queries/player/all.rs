use crate::entities::player::Player;
use crate::queries::player::PlayerAllInput;
use crate::Deps;
use std::sync::Arc;

struct ExecutorImpl {
    deps: Arc<Deps>,
}

pub fn new_executor(deps: Arc<Deps>) -> Box<dyn Executor> {
    Box::new(ExecutorImpl { deps })
}

#[async_trait::async_trait]
pub trait Executor: Send + Sync {
    async fn execute(&self, input: &PlayerAllInput) -> Result<Vec<Player>, String>;
}

#[async_trait::async_trait]
impl Executor for ExecutorImpl {
    async fn execute(&self, input: &PlayerAllInput) -> Result<Vec<Player>, String> {
        let all = self.deps.queries_repo.player_all(input).await?;

        Ok(all)
    }
}
