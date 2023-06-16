pub enum PorkemonEvent {
    PokemonCaptured(Captured),
    PokemonReleased(Released),
    PokemonFucked(Fucked),
}

pub struct Captured {}
pub struct Released {}
pub struct Fucked {}
