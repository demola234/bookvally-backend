use sqlx::{PgPool, Postgres, Transaction};

/// Begin a Postgres transaction. Caller must `.commit()` or drop to rollback.
pub async fn begin(pool: &PgPool) -> anyhow::Result<Transaction<'static, Postgres>> {
    let tx = pool.begin().await?;
    Ok(tx)
}
