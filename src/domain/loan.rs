use esrs::Aggregate;

use super::{
    command, error,
    event::{self, Authorized, LoanEvent, Setup},
};

pub struct LoanAggregate {}

#[derive(Clone, Debug)]
pub struct LoanState {
    pub status: String,
    pub name: String,
    pub bank_account: String,
    pub braintree_token: String,
}

impl LoanState {
    pub fn is_waiting_for_deposit(&self) -> bool {
        self.status == "Waiting for Deposit"
    }

    pub fn is_waiting_for_loan(&self) -> bool {
        self.status == "Loan Submitted"
    }

    pub fn is_deposit_payed(&self) -> bool {
        self.status == "Deposit Payed"
    }

    pub fn is_not_already_payed(&self) -> bool {
        self.status == "Setup"
    }

    pub fn captured(&self, name: String) -> Self {
        Self {
            status: "Captured".to_string(),
            name,
            bank_account: "".to_string(),
            braintree_token: "".to_string(),
        }
    }

    pub fn loan_submitted(&self) -> Self {
        Self {
            status: "Loan Submitted".to_string(),
            name: self.name.clone(),
            bank_account: "".to_string(),
            braintree_token: "".to_string(),
        }
    }

    pub fn loan_created(&self) -> Self {
        Self {
            status: "Loan Created".to_string(),
            name: self.name.clone(),
            bank_account: "".to_string(),
            braintree_token: "".to_string(),
        }
    }

    pub fn asked_for_deposit(&self) -> Self {
        Self {
            status: "Waiting for Deposit".to_string(),
            name: self.name.clone(),
            bank_account: "".to_string(),
            braintree_token: "".to_string(),
        }
    }

    pub fn deposit_payed(&self) -> Self {
        Self {
            status: "Deposit Payed".to_string(),
            name: self.name.clone(),
            bank_account: "".to_string(),
            braintree_token: "".to_string(),
        }
    }

    pub fn released(&self) -> Self {
        Self {
            status: "Released".to_string(),
            name: self.name.clone(),
            bank_account: "".to_string(),
            braintree_token: "".to_string(),
        }
    }

    fn setup(&self, bank_account: String, braintree_token: String) -> LoanState {
        Self {
            status: "Setup".to_string(),
            name: self.name.clone(),
            bank_account,
            braintree_token,
        }
    }
}

impl Default for LoanState {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            status: "Void".to_string(),
            bank_account: "".to_string(),
            braintree_token: "".to_string(),
        }
    }
}

impl Aggregate for LoanAggregate {
    const NAME: &'static str = "Pokemon";
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
            LoanEvent::PokemonReleased(_) => state.released(),
            LoanEvent::PokemonFucked(_) => todo!(),
            LoanEvent::AskedForDeposit => state.asked_for_deposit(),
            LoanEvent::DepositPayed => state.deposit_payed(),
            LoanEvent::LoanSubmitted => state.loan_submitted(),
            LoanEvent::LoanCreated => state.loan_created(),
        }
    }
}
