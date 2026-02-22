use colorize::AnsiColor;
use inquire::validator::{StringValidator, Validation};
use inquire::{Select, Text};
use lifeguard::quote::QuoteOutput;
use lifeguard::quote::tycho::{ChainSelection, measure_depth};
use strum::IntoEnumIterator;
use tokio;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let results: Vec<QuoteOutput> = vec![];

    let numeric_validator = |input: &str| match input.parse::<f64>() {
        Ok(num) => Ok(Validation::Valid),
        Err(input) => Ok(Validation::Invalid(
            "You need to input a numeric value.".into(),
        )),
    };

    loop {
        let chain = Select::new(
            &"Select your chain.".yellow(),
            ChainSelection::iter().collect(),
        )
        .prompt()?;
        let amount = Text::new(&"enter amount".yellow())
            .with_validator(numeric_validator)
            .prompt()?;
        let scalar = Text::new(&"enter scalar".yellow())
            .with_validator(numeric_validator)
            .prompt()?;
        )
    }
}
