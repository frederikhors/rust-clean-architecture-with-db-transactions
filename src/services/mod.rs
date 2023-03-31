pub mod commands;
pub mod queries;

pub struct App {
    pub commands: Commands,
    pub queries: Queries,
}

pub struct Commands {
    pub player_create: Box<dyn commands::player::create::Executor>,
    pub player_delete: Box<dyn commands::player::delete::Executor>,
    pub player_update: Box<dyn commands::player::update::Executor>,

    pub team_create: Box<dyn commands::team::create::Executor>,
}

pub struct Queries {
    pub player_by_id: Box<dyn queries::player::find::Executor>,
    pub player_all: Box<dyn queries::player::all::Executor>,

    pub team_by_id: Box<dyn queries::team::find::Executor>,
}
