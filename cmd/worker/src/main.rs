use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv().ok();
    // app::run_worker().await
    Ok(())
}
