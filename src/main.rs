use services::{
    commands::{self, player::PlayerInput},
    queries, App,
};
use std::sync::Arc;

pub mod entities;
pub mod repositories;
pub mod services;

pub struct Deps {
    pub commands_repo: Arc<dyn commands::RepoTrait>,
    pub queries_repo: Arc<dyn queries::RepoTrait>,
}

fn split_repo<T: commands::RepoTrait + queries::RepoTrait + 'static>(
    arc: Arc<T>,
) -> (Arc<dyn commands::RepoTrait>, Arc<dyn queries::RepoTrait>) {
    (arc.clone(), arc)
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let use_postgres = false;

    // This obviously works if alone:
    let db_repo = Arc::new(repositories::in_memory::Repo::new());

    // This obviously works if alone:
    // let pg_pool = Arc::new(sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/postgres").await.unwrap());
    // let db_repo = Arc::new(repositories::postgres::Repo::new(pg_pool));

    // This doesn't work instead:
    let (commands_repo, queries_repo) = if use_postgres {
        let pg_pool = Arc::new(
            sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/postgres")
                .await
                .unwrap(),
        );

        split_repo(Arc::new(repositories::postgres::Repo::new(pg_pool)))
    } else {
        split_repo(Arc::new(repositories::in_memory::Repo::new()))
    };

    let deps = Arc::new(Deps {
        commands_repo,
        queries_repo,
    });

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

    let new_player = app
        .commands
        .player_create
        .execute(&new_player_input)
        .await?;

    dbg!(&new_player);

    Ok(())
}
