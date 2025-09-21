use rusty_calculator::calc::{CalculationError, RustyCalculator};

fn main() -> Result<(), CalculationError>{
    match RustyCalculator::start() {
        Ok(_calc) => println!("Calculator finished successfully."),
        Err(e) => println!("Calculator error: {}", e),
    }
    Ok(())
}


