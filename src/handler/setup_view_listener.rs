use async_trait::async_trait;
use esrs::handler::EventHandler;
use esrs::store::StoreEvent;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::{event::PokemonEvent, pokemon::PokemonAggregate};

use super::setup_view::SetupView;

#[derive(Clone)]
pub struct SetupViewListener {
    pub pool: Pool<Postgres>,
    pub view: SetupView,
}

#[async_trait]
impl EventHandler<PokemonAggregate> for SetupViewListener {
    async fn handle(&self, event: &StoreEvent<PokemonEvent>) {
        match event.payload() {
            PokemonEvent::LoanAuthorized(payload) => {
                let _ = self
                    .view
                    .authorize(
                        event.aggregate_id,
                        payload.authorization_token,
                        payload.amount.to_string(),
                        payload.product.clone(),
                        &self.pool,
                    )
                    .await;
            }
            PokemonEvent::LoanSetup(payload) => {
                let _ = self
                    .view
                    .setup(
                        event.aggregate_id,
                        payload.nonce.clone(),
                        payload.bank_account.clone(),
                        payload.braintree_token.clone(),
                        &self.pool,
                    )
                    .await;
            }
            _ => (),
        }
    }

    async fn delete(&self, aggregate_id: Uuid) {
        if let Err(e) = self.view.delete(aggregate_id, &self.pool).await {
            eprintln!("Error while deleting view: {:?}", e);
        }
    }
}
