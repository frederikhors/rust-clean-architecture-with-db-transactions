use crate::{
    entities::team::Team,
    repositories::postgres::Repo,
    services::commands::{
        self,
        team::{create::TeamCreateLambdaArgs, TeamInput},
        Lambda,
    },
};

#[async_trait::async_trait]
impl commands::RepoTeam for Repo {
    async fn team_create(
        &self,
        input: &TeamInput,
        lambda: &Lambda<TeamCreateLambdaArgs, Team>,
    ) -> Result<Team, String> {
        println!("input: {:?} - team_create postgres repo", input);

        // create a transaction here because I can use it for other repository methods calls
        let tx = self.pool.begin().await.unwrap();

        // wait for lambda result
        let team = lambda(TeamCreateLambdaArgs {}).await?;

        // insert team here with appropriate code for this repository

        tx.commit().await.unwrap();

        Ok(team)
    }
}
