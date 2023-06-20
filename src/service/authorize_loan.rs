use std::sync::Arc;

use crate::domain::command::Authorize;
use crate::domain::command::Command::AuthorizeLoan;
use esrs::{manager::AggregateManager, store::postgres::PgStore, AggregateState};
use uuid::Uuid;

use crate::domain::{aggregate::LoanAggregate, loan_state::LoanState};

pub struct AuthorizeLoanService {
    pub manager: Arc<AggregateManager<PgStore<LoanAggregate>>>,
}
impl AuthorizeLoanService {
    pub async fn authorize(&self) -> Uuid {
        let aggregate_id = Uuid::new_v4();

        let state: AggregateState<LoanState> = AggregateState::with_id(aggregate_id);

        let authorization_token: Uuid = Uuid::new_v4();

        let authorize = AuthorizeLoan(Authorize {
            amount: 1000,
            product: "pol-1234".to_string(),
            authorization_token,
        });

        let _ = self.manager.handle_command(state, authorize).await;

        authorization_token
    }
}
