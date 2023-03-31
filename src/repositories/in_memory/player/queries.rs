use crate::entities::player::Player;
use crate::queries::player::PlayerAllInput;
use crate::{repositories::in_memory::Repo, services::queries::RepoPlayer};

#[async_trait::async_trait]
impl RepoPlayer for Repo {
    async fn player_by_id(&self, id: &str) -> Result<Option<Player>, String> {
        println!("id: {} - player_by_id in_memory repo", id);

        let obj = Player {
            ..Default::default()
        };

        Ok(Some(obj))
    }

    async fn player_all(&self, input: &PlayerAllInput) -> Result<Vec<Player>, String> {
        println!("input: {:?} - player_all in_memory repo", input);

        let all = vec![Player {
            ..Default::default()
        }];

        Ok(all)
    }
}
