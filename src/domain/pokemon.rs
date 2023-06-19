use esrs::Aggregate;

use super::{
    command, error,
    event::{self, Captured, PokemonEvent, Released},
};

pub struct PokemonAggregate {}

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
        let event = match command {
            command::Command::Capture(payload) => PokemonEvent::PokemonCaptured(Captured {
                nome_pokemon: payload.name,
            }),
            command::Command::Release(_payload) => PokemonEvent::PokemonReleased(Released {}),
            command::Command::Fuck(_) => todo!(),
        };
        Ok(vec![event])
    }

    fn apply_event(state: Self::State, event: Self::Event) -> Self::State {
        match event {
            PokemonEvent::PokemonCaptured(payload) => state.captured(payload.nome_pokemon),
            PokemonEvent::PokemonReleased(_) => state.released(),
            PokemonEvent::PokemonFucked(_) => todo!(),
        }
    }
}
