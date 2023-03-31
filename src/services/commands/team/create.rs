use crate::entities::team::Team;
use crate::services::commands::{team::TeamInput, RepoTrait};
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
    async fn execute(&self, input: &TeamInput) -> Result<Team, String>;
}

#[async_trait::async_trait]
impl<C: RepoTrait + Send + Sync> Executor for ExecutorImpl<C> {
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
