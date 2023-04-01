use services::{
    commands::{self, player::PlayerInput},
    queries, App,
};
use std::sync::Arc;

pub mod entities;
pub mod repositories;
pub mod services;

pub struct Deps<C> {
    pub commands_repo: Arc<C>,
    pub queries_repo: Arc<dyn queries::RepoTrait>,
}

fn create_app_with_repo<
    T: queries::RepoTeam + queries::RepoPlayer + commands::RepoTeam + commands::RepoPlayer + 'static,
>(
    repo: Arc<T>,
) -> App {
    let deps = Arc::new(Deps {
        commands_repo: repo.clone(),
        queries_repo: repo,
    });

    App {
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
    }
}

#[tokio::main]
async fn main() -> Result<(), String> {
    let use_postgres = false;

    let app = if use_postgres {
        let pg_pool = Arc::new(
            sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/postgres")
                .await
                .unwrap(),
        );

        create_app_with_repo(Arc::new(repositories::postgres::Repo::new(pg_pool)))
    } else {
        create_app_with_repo(Arc::new(repositories::in_memory::Repo::new()))
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
