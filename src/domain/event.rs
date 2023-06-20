use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum PokemonEvent {
    LoanAuthorized(Authorized),
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
pub struct Authorized {
    pub product: String,
    pub amount: u16,
    pub authorization_token: Uuid,
}
