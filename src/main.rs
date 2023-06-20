use esrs::AggregateState;
use pokemon::domain::command::Authorize;
use pokemon::domain::command::Command::AskForDeposit;
use pokemon::domain::command::Command::AuthorizeLoan;
use pokemon::domain::command::Command::SetDepositAsPayed;
use pokemon::domain::command::Command::SetupLoan;
use pokemon::domain::command::Setup;
use pokemon::domain::pokemon::PokemonAggregate;
use pokemon::domain::pokemon::PokemonState;
use pokemon::handler::authorization_view::AuthorizationView;
use pokemon::handler::authorization_view_listener::AuthorizationViewListener;
use pokemon::handler::setup_view::SetupView;
use pokemon::handler::setup_view_listener::SetupViewListener;
use sqlx::postgres::PgPoolOptions;

use sqlx::{Pool, Postgres};

use esrs::manager::AggregateManager;
use esrs::store::postgres::{PgStore, PgStoreBuilder};
use uuid::Uuid;

#[tokio::main]
async fn main() {
    let pool: Pool<Postgres> = new_pool().await;
    let setup_view = SetupView::new("SETUP_VIEW", &pool).await;
    let setup_listener = SetupViewListener {
        pool: pool.clone(),
        view: setup_view.clone(),
    };

    let view = AuthorizationView::new("AUTHORIZATION_VIEW", &pool).await;
    let auth_listener = AuthorizationViewListener {
        pool: pool.clone(),
        view: view.clone(),
    };

    let store: PgStore<PokemonAggregate> = PgStoreBuilder::new(pool.clone())
        .add_event_handler(auth_listener)
        .add_event_handler(setup_listener)
        .try_build()
        .await
        .unwrap();
    // Convalidare l'univocita' del comando
    // Come gestiamo la saga
    let manager = AggregateManager::new(store);

    let aggregate_id: Uuid = Uuid::new_v4();

    let auth_token = Steps::authorize(&manager, aggregate_id).await;
    let nonce = Steps::setup(&manager, &view, auth_token, pool.clone()).await;
    Steps::create_loan(&manager, &setup_view, nonce, pool.clone()).await;
}

struct Steps;

impl Steps {
    pub async fn authorize(
        manager: &AggregateManager<PgStore<PokemonAggregate>>,
        aggregate_id: Uuid,
    ) -> Uuid {
        let state: AggregateState<PokemonState> = AggregateState::with_id(aggregate_id);

        let authorization_token: Uuid = Uuid::new_v4();

        let authorize = AuthorizeLoan(Authorize {
            amount: 1000,
            product: "pol-1234".to_string(),
            authorization_token,
        });

        let _ = manager.handle_command(state, authorize).await;

        authorization_token
    }

    pub async fn setup(
        manager: &AggregateManager<PgStore<PokemonAggregate>>,
        view: &AuthorizationView,
        auth_token: Uuid,
        pool: Pool<Postgres>,
    ) -> Uuid {
        let row = view.by_token(auth_token, &pool).await.unwrap().unwrap();
        let state = manager.load(row.id).await.unwrap().unwrap();
        let nonce = Uuid::new_v4();
        let setup = SetupLoan(Setup {
            bank_account: "BCE".to_string(),
            braintree_nonce: "NONCE".to_string(),
            nonce,
        });

        let _ = manager.handle_command(state, setup).await;
        nonce
    }

    pub async fn create_loan(
        manager: &AggregateManager<PgStore<PokemonAggregate>>,
        view: &SetupView,
        nonce: Uuid,
        pool: Pool<Postgres>,
    ) -> Uuid {
        let row = view.by_nonce(nonce, &pool).await.unwrap().unwrap();

        let loan = manager.load(row.id).await.unwrap().unwrap();
        if loan.inner().is_not_already_payed() {
            let _ = manager.handle_command(loan, AskForDeposit).await;
        }

        let loan = manager.load(row.id).await.unwrap().unwrap();
        if loan.inner().is_waiting_for_deposit() {
            let _ = manager.handle_command(loan, SetDepositAsPayed).await;
        }

        let loan = manager.load(row.id).await.unwrap().unwrap();
        if loan.inner().is_deposit_payed() {
            let _ = manager.handle_command(loan, AskForLoan).await;
        }

        let loan = manager.load(row.id).await.unwrap().unwrap();
        if loan.inner().is_waiting_for_loan() {
            let _ = manager.handle_command(loan, SetLoanAsCreated).await;
        }
    }

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
}

pub async fn new_pool() -> Pool<Postgres> {
    PgPoolOptions::new()
        .connect("postgresql://postgres:postgres@localhost:5432/postgres")
        .await
        .unwrap()
}
