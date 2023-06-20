use sqlx::{Executor, Pool, Postgres};
use uuid::Uuid;

#[derive(sqlx::FromRow, Debug)]
pub struct AuthorizationViewRow {
    pub id: Uuid,
    pub content: String,
}

#[derive(Clone)]
pub struct AuthorizationView {
    table_name: String,
}

impl AuthorizationView {
    pub async fn new(table_name: &str, pool: &Pool<Postgres>) -> Self {
        let table_name: String = format!("{}_{}", "culo", table_name);

        let query: String = format!(
            "CREATE TABLE IF NOT EXISTS {} (id uuid PRIMARY KEY NOT NULL, token uuid, amount VARCHAR, product VARCHAR)",
            table_name
        );

        let _ = sqlx::query(query.as_str()).execute(pool).await.unwrap();

        Self { table_name }
    }

    pub async fn by_id(
        &self,
        id: Uuid,
        executor: impl Executor<'_, Database = Postgres>,
    ) -> Result<Option<AuthorizationViewRow>, sqlx::Error> {
        let query: String = format!("SELECT * FROM {} WHERE id = $1", &self.table_name);

        sqlx::query_as::<_, AuthorizationViewRow>(query.as_str())
            .bind(id)
            .fetch_optional(executor)
            .await
    }

    pub async fn upsert(
        &self,
        id: Uuid,
        token: Uuid,
        amount: String,
        product: String,
        executor: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), sqlx::Error> {
        let query = format!(
            "INSERT INTO {0} (id, token, amount, product) VALUES ($1, $2, $3,$4) ON CONFLICT (id) DO UPDATE SET token = $2, amount = $3, product =$4;",
            &self.table_name
        );

        sqlx::query(query.as_str())
            .bind(id)
            .bind(token)
            .bind(amount)
            .bind(product)
            .fetch_optional(executor)
            .await
            .map(|_| ())
    }

    pub async fn delete(
        &self,
        id: Uuid,
        executor: impl Executor<'_, Database = Postgres>,
    ) -> Result<(), sqlx::Error> {
        let query = format!("DELETE FROM {0} WHERE id = $1;", &self.table_name);

        sqlx::query(query.as_str())
            .bind(id)
            .fetch_optional(executor)
            .await
            .map(|_| ())
    }

    pub fn table_name(&self) -> &str {
        &self.table_name
    }
}
