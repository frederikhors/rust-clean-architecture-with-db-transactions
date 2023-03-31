use crate::entities::player::Player;
use crate::services::commands::player::PlayerInput;
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
    async fn execute(&self, input: &PlayerInput) -> Result<Player, String>;
}

#[async_trait::async_trait]
impl Executor for ExecutorImpl {
    async fn execute(&self, input: &PlayerInput) -> Result<Player, String> {
        let res = self
            .deps
            .commands_repo
            .player_create(input, &|_| {
                let input = input;

                Box::pin(async move {
                    let obj = Player {
                        id: "new_id".to_string(),
                        name: input.name.to_owned(),
                        team_id: input.team_id.to_owned(),
                    };

                    Ok(obj)
                })
            })
            .await?;
        Ok(res)
    }
}

pub struct PlayerCreateLambdaArgs {}
