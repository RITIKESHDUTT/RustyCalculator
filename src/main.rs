<<<<<<< HEAD
use rusty_calculator::calc::{CalculationError, RustyCalculator};

fn main() -> Result<(), CalculationError>{
=======
use calc_base::calc::{RustyCalculator, CalculationError};

fn main() -> Result<(), CalculationError> {
>>>>>>> origin/main
    match RustyCalculator::start() {
        Ok(_calc) => println!("Calculator finished successfully."),
        Err(e) => println!("Calculator error: {}", e),
    }
    Ok(())
}


