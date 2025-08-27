use strappier_service::run;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}
