use anyhow::bail;

pub struct Cash;

impl Cash {
    pub fn charge(&self, amount: String) -> anyhow::Result<()> {
        if amount == "1000" {
            println!("Paid : {} euros", amount);
            return Ok(());
        }
        bail!("Payment refused!")
    }
}
