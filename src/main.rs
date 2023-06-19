use esrs::AggregateState;
use pokemon::domain::command::Capture;
use pokemon::domain::command::Release;
use pokemon::domain::pokemon::PokemonAggregate;
use pokemon::domain::pokemon::PokemonState;
use sqlx::postgres::PgPoolOptions;

use sqlx::{Pool, Postgres};

use esrs::manager::AggregateManager;
use esrs::store::postgres::{PgStore, PgStoreBuilder};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let pool: Pool<Postgres> = new_pool().await;

    let store: PgStore<PokemonAggregate> =
        PgStoreBuilder::new(pool.clone()).try_build().await.unwrap();
    // Convalidare l'univocita' del comando
    // Come gestiamo la saga
    let manager = AggregateManager::new(store);
    let aggregate_id: Uuid = Uuid::new_v4();
    let state: AggregateState<PokemonState> = AggregateState::with_id(aggregate_id);

    let capture = pokemon::domain::command::Command::Capture(Capture {
        name: "Calogero".to_string(),
    });

    let result = manager.handle_command(state, capture).await;

    dbg!(&result);

    let state: AggregateState<PokemonState> = AggregateState::with_id(aggregate_id);

    let capture = pokemon::domain::command::Command::Capture(Capture {
        name: "Calogero".to_string(),
    });

    let _result = manager.handle_command(state, capture).await;
    let state = manager.load(aggregate_id).await.unwrap().unwrap();

    dbg!(&state);

    let command = pokemon::domain::command::Command::Release(Release {
        name: "Calogero".to_string(),
    });

    let _result = manager.handle_command(state, command).await;

    let state = manager.load(aggregate_id).await.unwrap().unwrap();

    dbg!(state);
}

pub async fn new_pool() -> Pool<Postgres> {
    PgPoolOptions::new()
        .connect("postgresql://postgres:postgres@localhost:5432/postgres")
        .await
        .unwrap()
}
