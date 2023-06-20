use uuid::Uuid;

pub enum Command {
    AuthorizeLoan(Authorize),
    SetupLoan(Setup),
    AskForDeposit,
    SetDepositAsPayed,
    AskForLoan,
    SetLoanAsCreated,
}

pub struct Authorize {
    pub amount: u16,
    pub product: String,
    pub authorization_token: Uuid,
}
pub struct Setup {
    pub bank_account: String,
    pub braintree_nonce: String,
    pub nonce: Uuid,
}
pub struct Buy {}
