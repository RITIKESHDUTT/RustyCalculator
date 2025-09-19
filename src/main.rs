use calc_base::calc::RustyCalculator;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    match RustyCalculator::start() {
        Ok(_calc) => println!("Calculator finished successfully."),
        Err(e) => println!("Calculator error: {}", e),
    }
    Ok(())
}


