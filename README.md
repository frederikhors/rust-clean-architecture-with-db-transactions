This is just an example of an _(still incomplete)_ real-world project written in Rust using a [clean architecture](https://blog.cleancoder.com/uncle-bob/2012/08/13/the-clean-architecture.html).

# Goals

My intent is to have an app build in 4 layers:

- `entities`:

  - _some call this layer "domain"_, not important for now, just the minimum

- `services`:

  - _some call this layer "use cases"_, this is where business logic lives (just CRUD methods for now)

- `repositories`:

  - _some call this layer "adapters"_, this is where concrete implementation of DB/cache/mail drivers lives

- `ports`:
  - _some call this layer "controllers or presenters"_, still not present and not important for now, I'm using `main.rs` for this

## Reproduction

https://codesandbox.io/p/github/frederikhors/rust-clean-architecture-with-db-transactions/main

# The issues

## Number 1

If you open the [`main.rs`](https://github.com/frederikhors/rust-clean-architecture-with-db-transactions/blob/main/src/main.rs#L18-L38) file you can see the first issue:

   <details>
   <summary>Expand the code</summary>

```rust
// This obviously works if alone:
// let db_repo = Arc::new(repositories::in_memory::Repo::new());

// This obviously works if alone:
// let pg_pool = Arc::new(sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/postgres").await.unwrap());
// let db_repo = Arc::new(repositories::postgres::Repo::new(pg_pool));

// This doesn't work instead:
let db_repo = if use_postgres {
    let pg_pool = Arc::new(sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/postgres").await.unwrap());

    Arc::new(repositories::postgres::Repo::new(pg_pool))
} else {
    Arc::new(repositories::in_memory::Repo::new())
};
```

   </details>

My intent here is to change repository in use based on a variable, but Rust doesn't like it, this is the error:

   <details>
   <summary>Expand the error</summary>

```
error[E0308]: `if` and `else` have incompatible types
--> src\main.rs:37:9
|
28 |       let db_repo = if use_postgres {
|  ___________________-
29 | |         let pg_pool = Arc::new(
30 | |             sqlx::PgPool::connect("postgres://postgres:postgres@localhost:5432/postgres")
31 | |                 .await
...  |
35 | |         Arc::new(repositories::postgres::Repo::new(pg_pool))
| |         ---------------------------------------------------- expected because of this
36 | |     } else {
37 | |         Arc::new(repositories::in_memory::Repo::new())
| |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `repositories::postgres::Repo`, found struct `in_memory::Repo`
38 | |     };
| |_____- `if` and `else` have incompatible types
|
= note: struct `in_memory::Repo` and struct `repositories::postgres::Repo` have similar names, but are actually distinct types
note: struct `in_memory::Repo` is defined in module `crate::repositories::in_memory` of the current crate
--> src\repositories\in_memory\mod.rs:6:1
|
6  | pub struct Repo {
| ^^^^^^^^^^^^^^^
note: struct `repositories::postgres::Repo` is defined in module `crate::repositories::postgres` of the current crate
--> src\repositories\postgres\mod.rs:6:1
|
6  | pub struct Repo {
| ^^^^^^^^^^^^^^^
```

   </details>

## Issue number 2

The second issue is about the [usage of a DB transaction in a service](https://github.com/frederikhors/rust-clean-architecture-with-db-transactions/blob/main/src/services/commands/player/update.rs#L26-L56) (of the same [bounded context](https://martinfowler.com/bliki/BoundedContext.html)):

   <details>
   <summary>Expand the code</summary>

```rust
 async fn execute(&self, input: &PlayerInput) -> Result<Player, String> {
     let player = self
         .deps
         .commands_repo
         .player_update(input, &|args| {
             Box::pin(async {
                 // I want to verify if there is any place for my player before updating it by using a method like the below
                 // but I wanna check this in a DB transaction

                 // I cannot pass transaction using lambda function because in the service layer I don't want to specify which DB I'm using and wich crate

                 // So one way to do this is by passing the team in the lambda args in `PlayerUpdateLambdaArgs`.

                 // The `team` is queried using the DB transaction on the repository level
                 // but as you can imagine this is a mess: I'm writing code here and there, back and forth

                 let team = self
                     .deps
                     .queries_repo
                     .team_by_id(&input.team_id)
                     .await
                     .unwrap();

                 if let Some(team) = team {
                     if team.missing_players == 0 {
                         return Err("no place for your player!".to_string());
                     }
                 }

                 let obj = Player {
                     id: args.actual.id,
                     name: input.name.to_owned(),
                     team_id: input.team_id.to_owned(),
                 };

                 Ok(obj)
             })
         })
         .await?;

     Ok(player)
 }
```

   </details>

As you can see I'm using a lambda function with a struct as argument because this is the only way I can fetch in the repository level the objects I need on the business logic level.

But as you can imagine the code is not linear and I have to go back & forth.

I think I should have something (but I don't know what) on the service layer to start (and commit/rollback) a DB transaction from there: but - as properly established by the rules of Clean architecture - the service layer cannot know the implementation details of the underlying levels (repositories).

I would like to use in my services something like (pseudo code):

```rust
// Start a new DB transaction now to use with the below methods

let transaction = [DONT_KNOW_HOW_PLEASE_START_A_NEW_DB_TRANSACTION]();

let team = self.repo.team_by_id(transaction, team_id).await?;

if !team.has_free_places() { return };

let mut player = self.repo.player_by_id(transaction, player_id).await?;

player.team_id = team.id;

let player = self.repo.player_update(player).await?;

Ok(player)
```

Is there a way to fix this?

Maybe yes and [there is a project](https://github.com/dpc/sniper) I found searching about this, but the code is too complex for me to completely understand how to do this in my project **and if there is something better** or even **if I'm wrong and why**.

The (maybe) interesting code is here: https://github.com/dpc/sniper/blob/master/src/persistence.rs.

**Another way** I found to fix this **is using state machines**. I created [a dedicated branch](https://github.com/frederikhors/rust-clean-architecture-with-db-transactions/tree/using-state-machines) with [one state machine usage for the `player_create` method](https://github.com/frederikhors/rust-clean-architecture-with-db-transactions/compare/using-state-machines?expand=1). like this:

   <details>
   <summary>Expand the code</summary>

```rust
// in the repository

pub struct PlayerCreate<'a> {
    tx: sqlx::Transaction<'a, sqlx::Postgres>,
    pub input: &'a PlayerInput,
}

#[async_trait::async_trait]
impl<'a> PlayerCreateTrait for PlayerCreate<'a> {
async fn check_for_team_free_spaces(&mut self, team_id: &str) -> Result<bool, String> {
let team = self::Repo::team_by_id_using_tx(&mut self.tx, team_id).await?;

        Ok(team.missing_players > 0)
    }

    async fn commit(mut self, _player: &Player) -> Result<Player, String> {
        // update the player here

        let saved_player = Player {
            ..Default::default()
        };

        self.tx.commit().await.unwrap();

        Ok(saved_player)
    }

}

#[async_trait::async_trait]
impl commands::RepoPlayer for Repo {
type PlayerCreate<'a> = PlayerCreate<'a>;

    async fn player_create_start<'a>(
        &self,
        input: &'a PlayerInput,
    ) -> Result<PlayerCreate<'a>, String> {
        let tx = self.pool.begin().await.unwrap();

        Ok(PlayerCreate { tx, input })
    }

}

// in the service

async fn execute(&self, input: &PlayerInput) -> Result<Player, String> {
let mut state_machine = self.deps.commands_repo.player_create_start(input).await?;

    if !(state_machine.check_for_team_free_spaces(&input.team_id)).await? {
        return Err("no free space available for this team".to_string());
    }

    let obj = Player {
        id: "new_id".to_string(),
        name: input.name.to_owned(),
        team_id: input.team_id.to_owned(),
    };

    let res = state_machine.commit(&obj).await?;

    Ok(res)

}

```

   </details>

But there are two big cons to this:

- a lot of code to write (also very repetitive);

- the same concepts must be used and repeated both in the repository layer and in the service layer or in any case the synthesis work to be done is not profitable for business logic but only for finding an intelligent way to avoid repeating code;

- you have to write repository methods which are very similar with the only difference that some take a db transaction as an argument and the other doesn't.

# Alternative ways

## Bloom legacy

I found the post: https://kerkour.com/rust-web-application-clean-architecture with the code here: https://github.com/skerkour/bloom-legacy.

I really like this code except for:

1. the service layer knows about repository implementation details

1. (and for this reason) it is impossible to change at runtime (or just to mock during tests) the DB driver.

I opened the issue: https://github.com/skerkour/bloom-legacy/issues/70 and I'm waiting for the author.

### Question

1. Can you help me solve issues 1 and 2?

1. Can you suggest an alternative way? I'm open to everything!

Thanks in advance.
