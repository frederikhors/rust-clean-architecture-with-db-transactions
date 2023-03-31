use services::{
    commands::{self, player::PlayerInput},
    queries, App,
};
use std::sync::Arc;

pub mod entities;
pub mod repositories;
pub mod services;

pub struct Deps<C: ?Sized> {
    pub commands_repo: Arc<C>,
    // pub commands_repo: Arc<dyn NewTrait>,
    // pub queries_repo: Arc<dyn queries::RepoTrait>,
    pub queries_repo: Arc<C>,
    // pub queries_repo: Arc<dyn NewTrait>,
}

// pub enum RepoEnum {
//     Memory(repositories::in_memory::Repo),
//     Postgres(repositories::postgres::Repo),
// }

// impl RepoTrait for RepoEnum {}
// impl RepoPlayer for RepoEnum {}
// impl RepoTeam for RepoEnum {}

pub trait NewTrait: services::queries::RepoTrait + services::commands::RepoTrait {}

#[tokio::main]
async fn main() -> Result<(), String> {
    let use_postgres = false;

    // This obviously works if alone:
    // let db_repo = Arc::new(repositories::in_memory::Repo::new());

    // This obviously works if alone:
    // let pg_pool = Arc::new(sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/postgres").await.unwrap());
    // let db_repo = Arc::new(repositories::postgres::Repo::new(pg_pool));

    // This doesn't work:
    // let db_repo = if use_postgres {
    //     // let db_repo: Arc<dyn commands::RepoTrait + queries::RepoTrait + Send + Sync + 'static> = if use_postgres {
    //     let pg_pool = Arc::new(
    //         sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/postgres")
    //             .await
    //             .unwrap(),
    //     );

    //     Arc::new(RepoEnum::Postgres(repositories::postgres::Repo::new(
    //         pg_pool,
    //     )))
    // } else {
    //     Arc::new(RepoEnum::Memory(repositories::in_memory::Repo::new()))
    // };

    // I'm trying with dyn Trait here:
    let db_repo: Arc<dyn NewTrait> = if use_postgres {
        let pg_pool = Arc::new(
            sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/postgres")
                .await
                .unwrap(),
        );

        Arc::new(repositories::postgres::Repo::new(pg_pool))
    } else {
        Arc::new(repositories::in_memory::Repo::new())
    };

    let deps = Arc::new(Deps {
        commands_repo: db_repo.clone(),
        queries_repo: db_repo,
    });

    let app = App {
        commands: {
            services::Commands {
                player_create: commands::player::create::new_executor(deps.clone()),
                player_delete: commands::player::delete::new_executor(deps.clone()),
                player_update: commands::player::update::new_executor(deps.clone()),
                team_create: commands::team::create::new_executor(deps.clone()),
            }
        },
        queries: {
            services::Queries {
                player_by_id: queries::player::find::new_executor(deps.clone()),
                player_all: queries::player::all::new_executor(deps.clone()),
                team_by_id: queries::team::find::new_executor(deps.clone()),
            }
        },
    };

    let new_player_input = PlayerInput {
        name: "Bob".to_string(),
        ..Default::default()
    };

    // let new_player = app
    //     .commands
    //     .player_create
    //     .execute(&new_player_input)
    //     .await?;

    // dbg!(&new_player);

    Ok(())
}
