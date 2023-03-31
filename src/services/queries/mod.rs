use crate::entities::{player::Player, team::Team};
use crate::queries::{player::PlayerAllInput, team::TeamAllInput};

pub mod player;
pub mod team;

pub trait RepoTrait: Send + Sync + RepoPlayer + RepoTeam {}

impl<T: RepoPlayer + RepoTeam> RepoTrait for T {}

#[async_trait::async_trait]
pub trait RepoPlayer: Send + Sync {
    async fn player_by_id(&self, id: &str) -> Result<Option<Player>, String>;

    async fn player_all(&self, input: &PlayerAllInput) -> Result<Vec<Player>, String>;
}

#[async_trait::async_trait]
pub trait RepoTeam: Send + Sync {
    async fn team_by_id(&self, id: &str) -> Result<Option<Team>, String>;

    async fn team_all(&self, input: &TeamAllInput) -> Result<Vec<Team>, String>;
}
