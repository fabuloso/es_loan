use esrs::Aggregate;

use crate::service::payer::Payer;

use super::{
    command, error,
    event::{self, Captured, PokemonEvent, Released},
};

pub struct PokemonAggregate {
    payer: Box<dyn Payer>,
}

#[derive(Clone, Debug)]
pub struct PokemonState {
    pub status: String,
    pub name: String,
}

impl PokemonState {
    pub fn captured(&self, name: String) -> Self {
        Self {
            status: "Captured".to_string(),
            name,
        }
    }

    pub fn loan_submitted(&self) -> Self {
        Self {
            status: "Loan Submitted".to_string(),
            name: self.name.clone(),
        }
    }

    pub fn loan_created(&self) -> Self {
        Self {
            status: "Loan Created".to_string(),
            name: self.name.clone(),
        }
    }

    pub fn asked_for_deposit(&self) -> Self {
        Self {
            status: "Waiting for Deposit".to_string(),
            name: self.name.clone(),
        }
    }

    pub fn deposit_payed(&self) -> Self {
        Self {
            status: "Deposit Payed".to_string(),
            name: self.name.clone(),
        }
    }

    pub fn released(&self) -> Self {
        Self {
            status: "Released".to_string(),
            name: self.name.clone(),
        }
    }
}

impl Default for PokemonState {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            status: "Pampurio".to_string(),
        }
    }
}

impl Aggregate for PokemonAggregate {
    const NAME: &'static str = "Pokemon";
    type State = PokemonState;
    type Command = command::Command;
    type Event = event::PokemonEvent;
    type Error = error::CommandError;

    fn handle_command(
        _state: &Self::State,
        command: Self::Command,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            command::Command::AuthorizeLoan(payload) => {
                Ok(vec![PokemonEvent::PokemonCaptured(Captured {
                    nome_pokemon: payload.name,
                })])
            }
            command::Command::SetupLoan(_payload) => {
                Ok(vec![PokemonEvent::PokemonReleased(Released {})])
            }
            command::Command::AskForDeposit => Ok(vec![PokemonEvent::AskedForDeposit]),
            command::Command::SetDepositAsPayed => Ok(vec![PokemonEvent::DepositPayed]),
            command::Command::CreateLoan => {
                Ok(vec![PokemonEvent::LoanSubmitted, PokemonEvent::LoanCreated])
            }
        }
    }

    fn apply_event(state: Self::State, event: Self::Event) -> Self::State {
        match event {
            PokemonEvent::PokemonCaptured(payload) => state.captured(payload.nome_pokemon),
            PokemonEvent::PokemonReleased(_) => state.released(),
            PokemonEvent::PokemonFucked(_) => todo!(),
            PokemonEvent::AskedForDeposit => state.asked_for_deposit(),
            PokemonEvent::DepositPayed => state.deposit_payed(),
            PokemonEvent::LoanSubmitted => state.loan_submitted(),
            PokemonEvent::LoanCreated => state.loan_created(),
        }
    }
}
