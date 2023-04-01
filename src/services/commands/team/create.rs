use crate::entities::team::Team;
use crate::services::commands::team::TeamInput;
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
    async fn execute(&self, input: &TeamInput) -> Result<Team, String>;
}

#[async_trait::async_trait]
impl Executor for ExecutorImpl {
    async fn execute(&self, input: &TeamInput) -> Result<Team, String> {
        let res = self
            .deps
            .commands_repo
            .team_create(input, &|_| {
                Box::pin(async move {
                    let obj = Team {
                        id: "new_id".to_string(),
                        name: input.name.to_owned(),
                        missing_players: 11,
                    };

                    Ok(obj)
                })
            })
            .await?;
        Ok(res)
    }
}

pub struct TeamCreateLambdaArgs {}
