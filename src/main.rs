use es_loan::domain::command::Authorize;
use es_loan::domain::command::Command::AskForDeposit;
use es_loan::domain::command::Command::AskForLoan;
use es_loan::domain::command::Command::AuthorizeLoan;
use es_loan::domain::command::Command::SetDepositAsPayed;
use es_loan::domain::command::Command::SetLoanAsCreated;
use es_loan::domain::command::Command::SetupLoan;
use es_loan::domain::command::Setup;
use es_loan::domain::loan::LoanAggregate;
use es_loan::domain::loan_state::LoanState;
use es_loan::handler::authorization_view::AuthorizationView;
use es_loan::handler::authorization_view_listener::AuthorizationViewListener;
use es_loan::handler::setup_view::SetupView;
use es_loan::handler::setup_view_listener::SetupViewListener;
use esrs::AggregateState;
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

    let store: PgStore<LoanAggregate> = PgStoreBuilder::new(pool.clone())
        .add_event_handler(auth_listener)
        .add_event_handler(setup_listener)
        .try_build()
        .await
        .unwrap();

    let manager = AggregateManager::new(store);

    let aggregate_id: Uuid = Uuid::new_v4();

    let auth_token = Steps::authorize(&manager, aggregate_id).await;
    let nonce = Steps::setup(&manager, &view, auth_token, pool.clone()).await;
    Steps::create_loan(&manager, &setup_view, nonce, pool.clone()).await;
}

struct Steps;

impl Steps {
    pub async fn authorize(
        manager: &AggregateManager<PgStore<LoanAggregate>>,
        aggregate_id: Uuid,
    ) -> Uuid {
        let state: AggregateState<LoanState> = AggregateState::with_id(aggregate_id);

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
        manager: &AggregateManager<PgStore<LoanAggregate>>,
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
        manager: &AggregateManager<PgStore<LoanAggregate>>,
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

        row.id
    }
}

pub async fn new_pool() -> Pool<Postgres> {
    PgPoolOptions::new()
        .connect("postgresql://postgres:postgres@localhost:5432/postgres")
        .await
        .unwrap()
}
