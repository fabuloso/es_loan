use esrs::Aggregate;

use super::{command, error, event};

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
    type Event = event::PokemonCaptured;
    type Error = error::CommandError;

    fn handle_command(
        _state: &Self::State,
        _command: Self::Command,
    ) -> Result<Vec<Self::Event>, Self::Error> {
        let event = event::PokemonCaptured {
            nome_pokemon: "Calogero".to_string(),
        };
        Ok(vec![event])
    }

    fn apply_event(_state: Self::State, _event: Self::Event) -> Self::State {
        Self::State {
            status: "Catturato".to_string(),
        }
    }
}
