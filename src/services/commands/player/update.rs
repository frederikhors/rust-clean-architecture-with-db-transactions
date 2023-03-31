use crate::entities::{player::Player, team::Team};
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
        let player = self
            .deps
            .commands_repo
            .player_update(input, &|args| {
                Box::pin(async {
                    // I want to verify if there is any place for my player before updating it by using a method like the below
                    // but I wanna check this in a DB transaction

                    // I cannot pass transaction using lambda function because in the service layer I don't want to specify which DB I'm using and wich crate

                    // So one way to do this is by passing the team in the lambda args in `PlayerUpdateLambdaArgs`.

                    // The `team` is queried using the DB transaction on the repository level
                    // but as you can imagine this is a mess: I'm writing code here and there, back and forth

                    let team = self
                        .deps
                        .queries_repo
                        .team_by_id(&input.team_id)
                        .await
                        .unwrap();

                    if let Some(team) = team {
                        if team.missing_players == 0 {
                            return Err("no place for your player!".to_string());
                        }
                    }

                    let obj = Player {
                        id: args.actual.id,
                        name: input.name.to_owned(),
                        team_id: input.team_id.to_owned(),
                    };

                    Ok(obj)
                })
            })
            .await?;

        Ok(player)
    }
}

pub struct PlayerUpdateLambdaArgs {
    pub actual: Player,
    pub actual_team: Team,
}
