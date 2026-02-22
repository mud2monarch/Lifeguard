use std::fmt;
use strum::{Display, EnumIter};

#[derive(Display, EnumIter)]
pub enum ChainSelection {
    Ethereum,
    Base,
    Unichain,
}

pub async fn measure_depth(amount: f64, scalar: f64, chain: ChainSelection) -> Result<f64, String> {
    let value = match chain {
        ChainSelection::Ethereum => 2.0,
        ChainSelection::Base => 4.0,
        _ => 1.0,
    };
    Ok(amount * 2.0 - value * scalar)
}
// use tycho_core::models::Chain;
// use tycho_simulation::utils::load_all_tokens;
// dotenvy::dotenv().ok();
//     let tycho_key = std::env::var("TYCHO_API_KEY").expect("TYCHO_API_KEY not set");

//     let eth_tokens = load_all_tokens(
//         "tycho-beta.propellerheads.xyz", // tycho url
//         false,                           // use tsl (this flag disables tsl)
//         Some(&tycho_key),                // auth key
//         true,
//         Chain::Ethereum, // chain
//         None,            // min quality (defaults to 100: ERC20-like tokens only)
//         Some(1),         // days since last trade (has chain specific defaults)
//     )
//     .await;
