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
pub trait PlayerCreateTrait {
    async fn check_for_team_free_spaces(&mut self, team_id: &str) -> Result<bool, String>;
    async fn commit(self, player: &Player) -> Result<Player, String>;
}

#[async_trait::async_trait]
impl<C: RepoTrait + Send + Sync> Executor for ExecutorImpl<C> {
    async fn execute(&self, input: &PlayerInput) -> Result<Player, String> {
        let mut state_machine = self.deps.commands_repo.player_create_start(input).await?;

        if !(state_machine.check_for_team_free_spaces(&input.team_id)).await? {
            return Err("no free space available for this team".to_string());
        }

        let obj = Player {
            id: "new_id".to_string(),
            name: input.name.to_owned(),
            team_id: input.team_id.to_owned(),
        };

        let res = state_machine.commit(&obj).await?;

        Ok(res)
    }
}

pub struct PlayerCreateLambdaArgs {}
