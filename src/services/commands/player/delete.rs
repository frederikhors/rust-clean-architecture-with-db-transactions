use crate::entities::player::Player;
use crate::services::commands::RepoTrait;
use crate::Deps;
use std::sync::Arc;

struct ExecutorImpl<C: ?Sized> {
    deps: Arc<Deps<C>>,
}

pub fn new_executor<C: RepoTrait + Send + Sync + 'static + ?Sized>(deps: Arc<Deps<C>>) -> Box<dyn Executor> {
    Box::new(ExecutorImpl { deps })
}

#[async_trait::async_trait]
pub trait Executor: Send + Sync {
    async fn execute(&self, id: &str) -> Result<bool, String>;
}

#[async_trait::async_trait]
impl<C: RepoTrait + Send + Sync + ?Sized> Executor for ExecutorImpl<C> {
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
