use crate::{
    entities::player::Player,
    repositories::in_memory::Repo,
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
        println!("input: {:?} - player_create in_memory repo", input);

        // create a transaction here because I can use it for other repository methods calls
        // let mut tx = self.pool.begin().await?;

        // wait for lambda result
        let player = lambda(PlayerCreateLambdaArgs {}).await?;

        // insert player here with appropriate code for this repository

        // commit DB transaction here
        // tx.commit().await?;

        Ok(player)
    }

    async fn player_delete(
        &self,
        id: &str,
        lambda: &Lambda<PlayerDeleteLambdaArgs, ()>,
    ) -> Result<(), String> {
        println!("id: {:?} - player_delete in_memory repo", id);

        // create a transaction here because I can use it for other repository methods calls
        // let mut tx = self.pool.begin().await?;

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

        // commit DB transaction here
        // tx.commit().await?;

        Ok(())
    }

    async fn player_update(
        &self,
        input: &PlayerInput,
        lambda: &Lambda<PlayerUpdateLambdaArgs, Player>,
    ) -> Result<Player, String> {
        println!("input: {:?} - player_update in_memory repo", input);

        // create a transaction here because I can use it for other repository methods calls
        // let mut tx = self.pool.begin().await?;

        // fetch current player here with appropriate code for this repository
        let actual = Player {
            ..Default::default()
        };

        // wait for lambda result
        let player = lambda(PlayerUpdateLambdaArgs { actual }).await?;

        // update player here with appropriate code for this repository

        // commit DB transaction here
        // tx.commit().await?;

        Ok(player)
    }
}
