use anyhow::Result;
use ethereum_stories::walletconnect;
use ethereum_stories::Scene;

#[tokio::main]
async fn main() -> Result<()> {
    dotenvy::dotenv()?;

    let scene = Scene {};

    let pubkey = walletconnect::WalletConnect::run().await;

    println!("Public Key: {pubkey:?}");

    scene.run().await;

    Ok(())
}
