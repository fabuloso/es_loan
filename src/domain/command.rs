pub enum Command {
    AuthorizeLoan(Authorize),
    SetupLoan(Setup),
    AskForDeposit,
    SetDepositAsPayed,
    CreateLoan,
}

pub struct Authorize {
    pub name: String,
}
pub struct Setup {
    pub name: String,
}
pub struct Buy {}
