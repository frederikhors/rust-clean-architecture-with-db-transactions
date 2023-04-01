use crate::entities::team::Team;
use crate::queries::team::TeamAllInput;
use crate::{repositories::postgres::Repo, services::queries::RepoTeam};

#[async_trait::async_trait]
impl RepoTeam for Repo {
    async fn team_by_id(&self, id: &str) -> Result<Option<Team>, String> {
        println!("id: {} - team_by_id postgres repo", id);

        let obj = Team {
            ..Default::default()
        };

        Ok(Some(obj))
    }

    async fn team_all(&self, input: &TeamAllInput) -> Result<Vec<Team>, String> {
        println!("input: {:?} - team_all postgres repo", input);

        let all = vec![Team {
            ..Default::default()
        }];

        Ok(all)
    }
}

impl Repo {
    pub async fn team_by_id_using_tx(
        _tx: &mut sqlx::PgConnection,
        _team_id: &str,
    ) -> Result<Team, String> {
        // fetch Team with DB transaction here, a fake one is returned now

        let team = Team {
            ..Default::default()
        };

        Ok(team)
    }
}
