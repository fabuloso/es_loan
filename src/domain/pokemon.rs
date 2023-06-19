use esrs::Aggregate;

use super::{
    command, error,
    event::{self, Captured, PokemonEvent},
};

pub struct PokemonAggregate {}

#[derive(Clone)]
pub struct PokemonState {
    pub status: String,
}

impl Default for PokemonState {
    fn default() -> Self {
        Self {
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
        _command: Self::Command,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        let event = PokemonEvent::PokemonCaptured(Captured {
            nome_pokemon: "Giacobbo".to_string(),
        });
        Ok(vec![event])
    }

    fn apply_event(_state: Self::State, _event: Self::Event) -> Self::State {
        Self::State {
            status: "Catturato".to_string(),
        }
    }
}
