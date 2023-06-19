use serde::{Deserialize, Serialize};

// pub enum PokemonEvent {
//     PokemonCaptured(Captured),
//     PokemonReleased(Released),
//     PokemonFucked(Fucked),
// }

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PokemonCaptured {
    pub nome_pokemon: String,
}
// #[derive(Serialize, Deserialize)]
// pub struct Released {}
// #[derive(Serialize, Deserialize)]
// pub struct Fucked {}
