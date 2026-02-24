use colorize::AnsiColor;
use inquire::validator::{StringValidator, Validation};
use inquire::{Select, Text};
use std::error::Error;
use std::fmt::Display;
use strum::IntoEnumIterator;

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

// TODO: loop until a valid api key is pasted
pub fn collect_api_key() -> Result<String, std::io::Error> {
    let key = Text::new(&"Paste in your Tycho key.".yellow()).prompt()?;
    Ok(key)
}

pub fn collect_input() -> Result<Vec<QuoteOutput>, Box<dyn Error>> {
    let results: Vec<QuoteOutput> = vec![];

    let numeric_validator = |input: &str| match input.parse::<f64>() {
        Ok(_) => Ok(Validation::Valid),
        Err(input) => Ok(Validation::Invalid(
            format!("You need to input a numeric value. You provided {}", input).into(),
        )),
    };

    loop {
        let amount = Text::new(&"enter amount".yellow())
            .with_validator(numeric_validator)
            .prompt()?;
        let scalar = Text::new(&"enter scalar".yellow())
            .with_validator(numeric_validator)
            .prompt()?;
    }
}
