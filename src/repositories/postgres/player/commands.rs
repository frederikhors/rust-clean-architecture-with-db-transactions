use crate::{
    entities::{player::Player, team::Team},
    repositories::postgres::Repo,
    services::commands::{
        self,
        player::{
            create::PlayerCreateTrait, delete::PlayerDeleteLambdaArgs,
            update::PlayerUpdateLambdaArgs, PlayerInput,
        },
        Lambda,
    },
};

pub struct PlayerCreate<'a> {
    tx: sqlx::Transaction<'a, sqlx::Postgres>,
    pub input: &'a PlayerInput,
}

#[async_trait::async_trait]
impl<'a> PlayerCreateTrait for PlayerCreate<'a> {
    async fn check_for_team_free_spaces(&mut self, team_id: &str) -> Result<bool, String> {
        let team = self::Repo::team_by_id_using_tx(&mut self.tx, team_id).await?;

        Ok(team.missing_players > 0)
    }

    async fn commit(mut self, _player: &Player) -> Result<Player, String> {
        // update the player here

        let saved_player = Player {
            ..Default::default()
        };

        self.tx.commit().await.unwrap();

        Ok(saved_player)
    }
}

#[async_trait::async_trait]
impl commands::RepoPlayer for Repo {
    type PlayerCreate<'a> = PlayerCreate<'a>;

    async fn player_create_start<'a>(
        &self,
        input: &'a PlayerInput,
    ) -> Result<PlayerCreate<'a>, String> {
        let tx = self.pool.begin().await.unwrap();

        Ok(PlayerCreate { tx, input })
    }

    async fn player_delete(
        &self,
        id: &str,
        lambda: &Lambda<PlayerDeleteLambdaArgs, ()>,
    ) -> Result<(), String> {
        println!("id: {:?} - player_delete postgres repo", id);

        // create a transaction here because I can use it for other repository methods calls
        let tx = self.pool.begin().await.unwrap();

        // fetch current player here with appropriate code for this repository
        let actual = Player {
            ..Default::default()
        };

        // wait for lambda result
        lambda(PlayerDeleteLambdaArgs {
            actual: actual.into(),
        })
        .await?;

        // delete player here with appropriate code for this repository

        tx.commit().await.unwrap();

        Ok(())
    }

    async fn player_update(
        &self,
        input: &PlayerInput,
        lambda: &Lambda<PlayerUpdateLambdaArgs, Player>,
    ) -> Result<Player, String> {
        println!("input: {:?} - player_update postgres repo", input);

        // create a transaction here because I can use it for other repository methods calls
        let tx = self.pool.begin().await.unwrap();

        // fetch current player here with appropriate code for this repository
        let actual = Player {
            ..Default::default()
        };

        // fetch current team here with appropriate code for this repository (a fake one now)
        let actual_team = Team {
            ..Default::default()
        };

        // wait for lambda result
        let player = lambda(PlayerUpdateLambdaArgs {
            actual,
            actual_team,
        })
        .await?;

        // update player here with appropriate code for this repository

        tx.commit().await.unwrap();

        Ok(player)
    }
}
