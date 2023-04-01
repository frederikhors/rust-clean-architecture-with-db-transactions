use crate::commands::RepoTeam;
use crate::entities::team::Team;
use crate::services::commands::team::TeamInput;
use crate::Deps;
use std::sync::Arc;

struct ExecutorImpl<C, Q> {
    deps: Arc<Deps<C, Q>>,
}

pub fn new_executor<C: Send + Sync + RepoTeam + 'static, Q: Send + Sync + 'static>(
    deps: Arc<Deps<C, Q>>,
) -> Box<dyn Executor> {
    Box::new(ExecutorImpl { deps })
}

#[async_trait::async_trait]
pub trait Executor: Send + Sync {
    async fn execute(&self, input: &TeamInput) -> Result<Team, String>;
}

#[async_trait::async_trait]
impl<C: Send + Sync + RepoTeam, Q: Send + Sync> Executor for ExecutorImpl<C, Q> {
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
