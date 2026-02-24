use lifeguard::quote::tycho::measure_depth;
use tokio::main;
use tycho_core::models::Chain;
use tycho_simulation::utils::load_all_tokens;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenvy::dotenv().ok();
    let tycho_key = std::env::var("TYCHO_API_KEY").expect("TYCHO_API_KEY not set");

    let eth_tokens = load_all_tokens(
        "tycho-beta.propellerheads.xyz", // tycho url
        false,                           // use tsl (this flag disables tsl)
        Some(&tycho_key),                // auth key
        true,
        Chain::Ethereum, // chain
        None,            // min quality (defaults to 100: ERC20-like tokens only)
        Some(1),         // days since last trade (has chain specific defaults)
    )
    .await;

    Ok(())
}
