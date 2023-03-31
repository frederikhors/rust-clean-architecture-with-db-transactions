use crate::entities::player::Player;
use crate::services::commands::{player::PlayerInput, RepoTrait};
use crate::Deps;
use std::sync::Arc;

struct ExecutorImpl<C> {
    deps: Arc<Deps<C>>,
}

pub fn new_executor<C: RepoTrait + Send + Sync + 'static>(deps: Arc<Deps<C>>) -> Box<dyn Executor> {
    Box::new(ExecutorImpl { deps })
}

#[async_trait::async_trait]
pub trait Executor: Send + Sync {
    async fn execute(&self, input: &PlayerInput) -> Result<Player, String>;
}

#[async_trait::async_trait]
impl<C: RepoTrait + Send + Sync> Executor for ExecutorImpl<C> {
    async fn execute(&self, input: &PlayerInput) -> Result<Player, String> {
        let res = self
            .deps
            .commands_repo
            .player_update(input, &|args| {
                Box::pin(async {
                    let obj = Player {
                        id: args.actual.id,
                        name: input.name.to_owned(),
                    };

                    Ok(obj)
                })
            })
            .await?;

        Ok(res)
    }
}

pub struct PlayerUpdateLambdaArgs {
    pub actual: Player,
}
