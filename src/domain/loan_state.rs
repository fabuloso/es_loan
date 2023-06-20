#[derive(Clone, Debug)]
pub struct LoanState {
    pub status: String,
    pub name: String,
    pub bank_account: String,
    pub braintree_token: String,
}

impl LoanState {
    pub fn is_waiting_for_deposit(&self) -> bool {
        self.status == "Waiting for Deposit"
    }

    pub fn is_waiting_for_loan(&self) -> bool {
        self.status == "Loan Submitted"
    }

    pub fn is_deposit_payed(&self) -> bool {
        self.status == "Deposit Payed"
    }

    pub fn is_not_already_payed(&self) -> bool {
        self.status == "Setup"
    }

    pub fn captured(&self, name: String) -> Self {
        Self {
            status: "Captured".to_string(),
            name,
            bank_account: "".to_string(),
            braintree_token: "".to_string(),
        }
    }

    pub fn loan_submitted(&self) -> Self {
        Self {
            status: "Loan Submitted".to_string(),
            name: self.name.clone(),
            bank_account: "".to_string(),
            braintree_token: "".to_string(),
        }
    }

    pub fn loan_created(&self) -> Self {
        Self {
            status: "Loan Created".to_string(),
            name: self.name.clone(),
            bank_account: "".to_string(),
            braintree_token: "".to_string(),
        }
    }

    pub fn asked_for_deposit(&self) -> Self {
        Self {
            status: "Waiting for Deposit".to_string(),
            name: self.name.clone(),
            bank_account: "".to_string(),
            braintree_token: "".to_string(),
        }
    }

    pub fn deposit_payed(&self) -> Self {
        Self {
            status: "Deposit Payed".to_string(),
            name: self.name.clone(),
            bank_account: "".to_string(),
            braintree_token: "".to_string(),
        }
    }

    pub fn released(&self) -> Self {
        Self {
            status: "Released".to_string(),
            name: self.name.clone(),
            bank_account: "".to_string(),
            braintree_token: "".to_string(),
        }
    }

    pub fn setup(&self, bank_account: String, braintree_token: String) -> LoanState {
        Self {
            status: "Setup".to_string(),
            name: self.name.clone(),
            bank_account,
            braintree_token,
        }
    }
}

impl Default for LoanState {
    fn default() -> Self {
        Self {
            name: "".to_string(),
            status: "Void".to_string(),
            bank_account: "".to_string(),
            braintree_token: "".to_string(),
        }
    }
}
