use esrs::AggregateState;
use pokemon::domain::command::Capture;
use pokemon::domain::pokemon::PokemonAggregate;
use pokemon::domain::pokemon::PokemonState;
use sqlx::postgres::PgPoolOptions;

use sqlx::{Pool, Postgres};

use esrs::manager::AggregateManager;
use esrs::store::postgres::{PgStore, PgStoreBuilder};

#[tokio::main]
async fn main() {
    let pool: Pool<Postgres> = new_pool().await;

    let store: PgStore<PokemonAggregate> =
        PgStoreBuilder::new(pool.clone()).try_build().await.unwrap();

    let command = pokemon::domain::command::Command::Capture(Capture {});

    let manager = AggregateManager::new(store);

    let state: AggregateState<PokemonState> = AggregateState::new();

    let result = manager.handle_command(state, command).await;

    dbg!(&result);
}

pub async fn new_pool() -> Pool<Postgres> {
    PgPoolOptions::new()
        .connect("postgresql://postgres:postgres@localhost:5432/postgres")
        .await
        .unwrap()
}
