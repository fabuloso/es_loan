use esrs::AggregateState;
use pokemon::domain::command::Authorize;
use pokemon::domain::command::Command::AskForDeposit;
use pokemon::domain::command::Command::AuthorizeLoan;
use pokemon::domain::command::Command::CreateLoan;
use pokemon::domain::command::Command::SetDepositAsPayed;
use pokemon::domain::pokemon::PokemonAggregate;
use pokemon::domain::pokemon::PokemonState;
use pokemon::handler::authorization_view::AuthorizationView;
use pokemon::handler::authorization_view_listener::AuthorizationViewListener;
use sqlx::postgres::PgPoolOptions;

use sqlx::{Pool, Postgres};

use esrs::manager::AggregateManager;
use esrs::store::postgres::{PgStore, PgStoreBuilder};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let pool: Pool<Postgres> = new_pool().await;
    let view = AuthorizationView::new("AUTHORIZATION_VIEW", &pool).await;
    let auth_listener = AuthorizationViewListener {
        pool: pool.clone(),
        view,
    };
    let store: PgStore<PokemonAggregate> = PgStoreBuilder::new(pool.clone())
        .add_event_handler(auth_listener)
        .try_build()
        .await
        .unwrap();
    // Convalidare l'univocita' del comando
    // Come gestiamo la saga
    let manager = AggregateManager::new(store);

    let aggregate_id: Uuid = Uuid::new_v4();
    let state: AggregateState<PokemonState> = AggregateState::with_id(aggregate_id);

    let authorization_token: Uuid = Uuid::new_v4();

    let authorize = AuthorizeLoan(Authorize {
        amount: 1000,
        product: "pol-1234".to_string(),
        authorization_token,
    });

    let _ = manager.handle_command(state, authorize).await;

    Steps::pay_deposit(&manager, aggregate_id).await;
    Steps::create_loan(&manager, aggregate_id).await;
}

struct Steps;

impl Steps {
    pub async fn pay_deposit(
        manager: &AggregateManager<PgStore<PokemonAggregate>>,
        aggregate_id: Uuid,
    ) {
        Self::ask_for_deposit(manager, aggregate_id).await;
        Self::set_as_payed_deposit(manager, aggregate_id).await;
    }

    async fn ask_for_deposit(
        manager: &AggregateManager<PgStore<PokemonAggregate>>,
        aggregate_id: Uuid,
    ) {
        let state = manager.load(aggregate_id).await.unwrap().unwrap();

        dbg!(&state);

        let command = AskForDeposit;

        let _ = manager.handle_command(state, command).await;
    }

    async fn set_as_payed_deposit(
        manager: &AggregateManager<PgStore<PokemonAggregate>>,
        aggregate_id: Uuid,
    ) {
        let state = manager.load(aggregate_id).await.unwrap().unwrap();

        dbg!(&state);

        let command = SetDepositAsPayed;

        let _ = manager.handle_command(state, command).await;
    }

    pub async fn create_loan(
        manager: &AggregateManager<PgStore<PokemonAggregate>>,
        aggregate_id: Uuid,
    ) {
        let state = manager.load(aggregate_id).await.unwrap().unwrap();

        dbg!(&state);

        let command = CreateLoan;

        let result = manager.handle_command(state, command).await;
    }
}

pub async fn new_pool() -> Pool<Postgres> {
    PgPoolOptions::new()
        .connect("postgresql://postgres:postgres@localhost:5432/postgres")
        .await
        .unwrap()
}
