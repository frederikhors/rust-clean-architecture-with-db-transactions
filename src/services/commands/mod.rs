use self::player::create::PlayerCreateTrait;
use self::player::delete::PlayerDeleteLambdaArgs;
use self::player::update::PlayerUpdateLambdaArgs;
use self::team::create::TeamCreateLambdaArgs;
use crate::entities::player::Player;
use crate::entities::team::Team;
use crate::services::commands::{player::PlayerInput, team::TeamInput};
use std::{future::Future, pin::Pin};

pub mod player;
pub mod team;

pub trait RepoTrait: Send + Sync + RepoPlayer + RepoTeam {}

impl<T: RepoPlayer + RepoTeam> RepoTrait for T {}

pub type Lambda<'a, ArgT, ResT> =
    dyn 'a + Fn(ArgT) -> Pin<Box<dyn Future<Output = Result<ResT, String>> + Send + 'a>> + Sync;

#[async_trait::async_trait]
pub trait RepoPlayer: Send + Sync {
    type PlayerCreate<'a>: Send + PlayerCreateTrait;

    async fn player_create_start<'a>(
        &self,
        input: &'a PlayerInput,
    ) -> Result<Self::PlayerCreate<'a>, String>;

    async fn player_delete<'a>(
        &'a self,
        id: &str,
        lambda: &Lambda<PlayerDeleteLambdaArgs, ()>,
    ) -> Result<(), String>;

    async fn player_update<'a>(
        &'a self,
        input: &PlayerInput,
        lambda: &Lambda<PlayerUpdateLambdaArgs, Player>,
    ) -> Result<Player, String>;
}

#[async_trait::async_trait]
pub trait RepoTeam: Send + Sync {
    async fn team_create<'a>(
        &'a self,
        input: &TeamInput,
        lambda: &Lambda<TeamCreateLambdaArgs, Team>,
    ) -> Result<Team, String>;
}