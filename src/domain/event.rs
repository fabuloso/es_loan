use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PokemonEvent {
    PokemonCaptured(Captured),
    PokemonReleased(Released),
    PokemonFucked(Fucked),
    AskedForDeposit,
    DepositPayed,
    LoanSubmitted,
    LoanCreated,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Released {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Fucked {}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Captured {
    pub nome_pokemon: String,
}
