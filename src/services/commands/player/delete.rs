use crate::commands::RepoPlayer;
use crate::entities::player::Player;
use crate::Deps;
use std::sync::Arc;

struct ExecutorImpl<C, Q> {
    deps: Arc<Deps<C, Q>>,
}

pub fn new_executor<C: Send + Sync + RepoPlayer + 'static, Q: Send + Sync + 'static>(
    deps: Arc<Deps<C, Q>>,
) -> Box<dyn Executor> {
    Box::new(ExecutorImpl { deps })
}

#[async_trait::async_trait]
pub trait Executor: Send + Sync {
    async fn execute(&self, id: &str) -> Result<bool, String>;
}

#[async_trait::async_trait]
impl<C: Send + Sync + RepoPlayer, Q: Send + Sync> Executor for ExecutorImpl<C, Q> {
    async fn execute(&self, id: &str) -> Result<bool, String> {
        self.deps
            .commands_repo
            .player_delete(id, &|_actual| Box::pin(async { Ok(()) }))
            .await?;

        let res = true;

        Ok(res)
    }
}

pub struct PlayerDeleteLambdaArgs {
    pub actual: Player,
}
