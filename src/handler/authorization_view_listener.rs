use async_trait::async_trait;
use esrs::handler::EventHandler;
use esrs::store::StoreEvent;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use crate::domain::{event::LoanEvent, loan::LoanAggregate};

use super::authorization_view::AuthorizationView;

#[derive(Clone)]
pub struct AuthorizationViewListener {
    pub pool: Pool<Postgres>,
    pub view: AuthorizationView,
}

#[async_trait]
impl EventHandler<LoanAggregate> for AuthorizationViewListener {
    async fn handle(&self, event: &StoreEvent<LoanEvent>) {
        match event.payload() {
            LoanEvent::LoanAuthorized(payload) => {
                let _ = self
                    .view
                    .upsert(
                        event.aggregate_id,
                        payload.authorization_token,
                        payload.amount.to_string(),
                        payload.product.clone(),
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
