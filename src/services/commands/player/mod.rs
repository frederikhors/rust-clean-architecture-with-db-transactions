pub mod create;
pub mod delete;
pub mod update;

#[derive(Debug, Default)]
pub struct PlayerInput {
    pub name: String,
    pub team_id: String,
}
