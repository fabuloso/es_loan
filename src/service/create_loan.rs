use crate::domain::command::Command::AskForDeposit;
use crate::domain::command::Command::AskForLoan;
use crate::domain::command::Command::SetDepositAsPayed;
use crate::domain::command::Command::SetLoanAsCreated;
use std::sync::Arc;

use esrs::{manager::AggregateManager, store::postgres::PgStore};
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::{domain::aggregate::LoanAggregate, handler::setup_view::SetupView};

use super::cash::Cash;

pub struct CreateLoanService {
    pub manager: Arc<AggregateManager<PgStore<LoanAggregate>>>,
    pub view: SetupView,
    pub pool: Pool<Postgres>,
    pub cash: Cash,
}

impl CreateLoanService {
    pub async fn create_loan(&self, nonce: Uuid) -> anyhow::Result<Uuid> {
        let row = self
            .view
            .by_nonce(nonce, &self.pool)
            .await
            .unwrap()
            .unwrap();

        let loan = self.manager.load(row.id).await.unwrap().unwrap();
        if loan.inner().is_not_already_payed() {
            let _ = self.manager.handle_command(loan, AskForDeposit).await;
        }

        if let Err(_) = self.cash.charge(row.amount) {
            anyhow::bail!("Houston we got a problem!");
        }

        let loan = self.manager.load(row.id).await.unwrap().unwrap();
        if loan.inner().is_waiting_for_deposit() {
            let _ = self.manager.handle_command(loan, SetDepositAsPayed).await;
        }

        let loan = self.manager.load(row.id).await.unwrap().unwrap();
        if loan.inner().is_deposit_payed() {
            let _ = self.manager.handle_command(loan, AskForLoan).await;
        }

        let loan = self.manager.load(row.id).await.unwrap().unwrap();
        if loan.inner().is_waiting_for_loan() {
            let _ = self.manager.handle_command(loan, SetLoanAsCreated).await;
        }

        Ok(row.id)
    }
}
