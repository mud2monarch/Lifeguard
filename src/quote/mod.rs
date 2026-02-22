use std::fmt::Display;

pub mod tycho;

#[derive(Debug)]
pub struct QuoteOutput {
    amount: f64,
    token: String,
    timestamp: i64,
}

impl Display for QuoteOutput {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Output was {} of {} token. Measured at {}.",
            self.amount, self.token, self.timestamp
        )
    }
}
