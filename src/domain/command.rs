use uuid::Uuid;

pub enum Command {
    AuthorizeLoan(Authorize),
    SetupLoan(Setup),
    AskForDeposit,
    SetDepositAsPayed,
    CreateLoan,
}

pub struct Authorize {
    pub amount: u16,
    pub product: String,
    pub authorization_token: Uuid,
}
pub struct Setup {
    pub name: String,
}
pub struct Buy {}
