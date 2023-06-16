use esrs::Aggregate;

use super::{command, error, event};

pub struct PorkemonAggregate {}
pub struct PorkemonState {
    pub status: String,
}

impl Aggregate for PorkemonAggregate {
    const NAME: &'static str = "Porkemon";

    type State = PorkemonState;

    type Command = command::Command;

    type Event = event::PorkemonEvent;

    type Error = error::CommandError;

    fn handle_command(
        _state: &Self::State,
        command: Self::Command,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        match command {
            command::Command::Capture(_) => {
                let event = event::PorkemonEvent::PokemonCaptured(event::Captured {});
                Ok(vec![event])
            }
            _ => todo!(),
        }
    }

    fn apply_event(_state: Self::State, event: Self::Event) -> Self::State {
        match event {
            event::PorkemonEvent::PokemonCaptured(_) => PorkemonState {
                status: "Catturato".to_string(),
            },
            _ => todo!(),
        }
    }
}

impl Default for PorkemonState {
    fn default() -> Self {
        Self {
            status: "Pampurio".to_string(),
        }
    }
}
