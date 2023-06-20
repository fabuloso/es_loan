use std::sync::Arc;

use crate::domain::command::Setup;
use crate::{domain::command::Command::SetupLoan, handler::authorization_view::AuthorizationView};
use esrs::{manager::AggregateManager, store::postgres::PgStore, AggregateState};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::{aggregate::LoanAggregate, loan_state::LoanState};

pub struct SetupLoanService {
    pub manager: Arc<AggregateManager<PgStore<LoanAggregate>>>,
    pub view: AuthorizationView,
    pub pool: Pool<Postgres>,
}
impl SetupLoanService {
    pub async fn setup(&self, auth_token: Uuid) -> Uuid {
        let row = self
            .view
            .by_token(auth_token, &self.pool)
            .await
            .unwrap()
            .unwrap();
        let state = self.manager.load(row.id).await.unwrap().unwrap();
        let nonce = Uuid::new_v4();

        let setup = SetupLoan(Setup {
            bank_account: "BCE".to_string(),
            braintree_nonce: "NONCE".to_string(),
            nonce,
        });

        let _ = self.manager.handle_command(state, setup).await;
        nonce
    }
}
