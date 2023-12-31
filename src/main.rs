use std::sync::Arc;
use std::{thread, time};

use es_loan::domain::aggregate::LoanAggregate;
use es_loan::handler::authorization_view::AuthorizationView;
use es_loan::handler::authorization_view_listener::AuthorizationViewListener;
use es_loan::handler::setup_view::SetupView;
use es_loan::handler::setup_view_listener::SetupViewListener;
use es_loan::service::authorize_loan::AuthorizeLoanService;
use es_loan::service::cash::Cash;
use es_loan::service::create_loan::CreateLoanService;
use es_loan::service::setup_loan::SetupLoanService;
use sqlx::postgres::PgPoolOptions;

use sqlx::{Pool, Postgres};

use esrs::manager::AggregateManager;
use esrs::store::postgres::{PgStore, PgStoreBuilder};

#[tokio::main]
async fn main() {
    let pool: Pool<Postgres> = new_pool().await;
    let setup_view = SetupView::new("SETUP_VIEW", &pool).await;
    let view = AuthorizationView::new("AUTHORIZATION_VIEW", &pool).await;
    let store = event_store(pool.clone(), setup_view.clone(), view.clone()).await;
    let manager = Arc::new(AggregateManager::new(store));

    let authorize_service = AuthorizeLoanService {
        manager: manager.clone(),
    };

    let setup_service = SetupLoanService {
        manager: manager.clone(),
        view,
        pool: pool.clone(),
    };

    let create_loan_service = CreateLoanService {
        manager: manager.clone(),
        view: setup_view,
        pool: pool.clone(),
        cash: Cash,
    };

    let token = authorize_service
        .authorize(1222, "policy-2".to_string())
        .await;

    thread::sleep(time::Duration::from_secs(15));

    let nonce = setup_service
        .setup(token, "My_Bank".to_string())
        .await
        .unwrap();

    let loan_id = create_loan_service.create_loan(nonce).await.unwrap();

    println!("LOAN_CREATED: {}", loan_id)
}

pub async fn event_store(
    pool: Pool<Postgres>,
    setup_view: SetupView,
    authorize_view: AuthorizationView,
) -> PgStore<LoanAggregate> {
    let setup_listener = SetupViewListener {
        pool: pool.clone(),
        view: setup_view.clone(),
    };
    let auth_listener = AuthorizationViewListener {
        pool: pool.clone(),
        view: authorize_view.clone(),
    };

    PgStoreBuilder::new(pool.clone())
        .add_event_handler(auth_listener)
        .add_event_handler(setup_listener)
        .try_build()
        .await
        .unwrap()
}

pub async fn new_pool() -> Pool<Postgres> {
    PgPoolOptions::new()
        .connect("postgresql://postgres:postgres@localhost:5432/postgres")
        .await
        .unwrap()
}
