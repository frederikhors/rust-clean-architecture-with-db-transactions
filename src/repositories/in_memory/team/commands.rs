use crate::{
    entities::team::Team,
    repositories::in_memory::Repo,
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
        println!("input: {:?} - team_create in_memory repo", input);

        // create a transaction here because I can use it for other repository methods calls
        // let mut tx = self.pool.begin().await?;

        // wait for lambda result
        let team = lambda(TeamCreateLambdaArgs {}).await?;

        // insert team here with appropriate code for this repository

        // commit DB transaction here
        // tx.commit().await?;

        Ok(team)
    }
}
