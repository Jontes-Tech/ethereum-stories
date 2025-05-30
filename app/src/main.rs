use anyhow::Result;
use ethereum_stories::Scene;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let scene = Scene {};

    scene.run();

    Ok(())
}
