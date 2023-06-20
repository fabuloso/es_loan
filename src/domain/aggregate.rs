use esrs::Aggregate;

use super::{
    command, error,
    event::{self, Authorized, LoanEvent, Setup},
    loan_state::LoanState,
};

pub struct LoanAggregate {}

impl Aggregate for LoanAggregate {
    const NAME: &'static str = "Loan";
    type State = LoanState;
    type Command = command::Command;
    type Event = event::LoanEvent;
    type Error = error::CommandError;

    fn handle_command(
        _state: &Self::State,
        command: Self::Command,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            command::Command::AuthorizeLoan(payload) => {
                Ok(vec![LoanEvent::LoanAuthorized(Authorized {
                    product: payload.product,
                    amount: payload.amount,
                    authorization_token: payload.authorization_token,
                })])
            }
            command::Command::SetupLoan(payload) => Ok(vec![LoanEvent::LoanSetup(Setup {
                bank_account: payload.bank_account,
                braintree_token: payload.braintree_nonce,
                nonce: payload.nonce,
            })]),
            command::Command::AskForDeposit => Ok(vec![LoanEvent::AskedForDeposit]),
            command::Command::SetDepositAsPayed => Ok(vec![LoanEvent::DepositPayed]),
            command::Command::AskForLoan => Ok(vec![LoanEvent::LoanSubmitted]),
            command::Command::SetLoanAsCreated => Ok(vec![LoanEvent::LoanCreated]),
        }
    }

    fn apply_event(state: Self::State, event: Self::Event) -> Self::State {
        match event {
            LoanEvent::LoanAuthorized(payload) => state.captured(payload.product),
            LoanEvent::LoanSetup(payload) => {
                state.setup(payload.bank_account, payload.braintree_token)
            }
            LoanEvent::AskedForDeposit => state.asked_for_deposit(),
            LoanEvent::DepositPayed => state.deposit_payed(),
            LoanEvent::LoanSubmitted => state.loan_submitted(),
            LoanEvent::LoanCreated => state.loan_created(),
        }
    }
}
