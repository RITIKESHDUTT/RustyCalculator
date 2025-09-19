use crate::calc::CalculationError;
pub trait LogicOperations {
    fn add(&mut self, value: f64) -> Result<(),CalculationError>;
    fn multiply(&mut self, value: f64) ->  Result<(),CalculationError>;
    fn divide(&mut self, value: f64) -> Result<(), CalculationError>;
    fn subtract(&mut self, value: f64)-> Result<(), CalculationError>;
    fn exp(&mut self, value: f64)-> Result<(), CalculationError>;
    fn square_root(&mut self)-> Result<(), CalculationError>;
    fn square(&mut self)-> Result<(), CalculationError>;
    fn natural_log(&mut self)-> Result<(), CalculationError>;
}
