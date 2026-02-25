use lifeguard::quote::tycho::measure_depth;
use tokio::main;
use tycho_client::feed::component_tracker::ComponentFilter;
use tycho_core::models::Chain;
use tycho_simulation::{
    evm::engine_db::tycho_db::PreCachedDB, evm::protocol::uniswap_v3::state::UniswapV3State,
    evm::protocol::vm::state::EVMPoolState, evm::stream::ProtocolStreamBuilder,
    utils::load_all_tokens,
};

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

    let tvl_filter = ComponentFilter::with_tvl_range(2000.0, 15000.0);
    let mut protocol_stream =
        ProtocolStreamBuilder::new("tycho-beta.propellerheads.xyz", Chain::Ethereum)
            .exchange::<UniswapV3State>("uniswap_v3", tvl_filter.clone(), None)
            // add other protocols here
            .auth_key(Some(tycho_key))
            .skip_state_decode_failures(true) // skips the pool instead of panicking if it errors on decode
            .set_tokens(eth_tokens.clone())
            .await
            .build()
            .await
            .expect("Failed building protocol stream");

    Ok(())
}
