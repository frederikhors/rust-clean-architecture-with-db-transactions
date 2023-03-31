use crate::{
    entities::{player::Player, team::Team},
    repositories::postgres::Repo,
    services::commands::{
        self,
        player::{
            create::PlayerCreateLambdaArgs, delete::PlayerDeleteLambdaArgs,
            update::PlayerUpdateLambdaArgs, PlayerInput,
        },
        Lambda,
    },
};

#[async_trait::async_trait]
impl commands::RepoPlayer for Repo {
    async fn player_create(
        &self,
        input: &PlayerInput,
        lambda: &Lambda<PlayerCreateLambdaArgs, Player>,
    ) -> Result<Player, String> {
        println!("input: {:?} - player_create postgres repo", input);

        // create a transaction here because I can use it for other repository methods calls
        let tx = self.pool.begin().await.unwrap();

        // wait for lambda result
        let player = lambda(PlayerCreateLambdaArgs {}).await?;

        // insert player here with appropriate code for this repository

        tx.commit().await.unwrap();

        Ok(player)
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
