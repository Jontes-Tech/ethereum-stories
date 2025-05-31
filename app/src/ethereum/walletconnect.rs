pub struct WalletConnect {}

impl WalletConnect {
    pub async fn run() -> Result<String, anyhow::Error> {
        println!("Running WalletConnect...");

        Ok("0x1234567890abcdef1234567890abcdef12345678".to_string())
    }
}
