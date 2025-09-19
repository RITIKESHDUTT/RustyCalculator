use crate::calc::CalculationError;

pub trait GeneralOperations {
    fn input(&mut self, value: f64);
    fn output(&self);
    fn delete(&mut self) -> Result<(), CalculationError> ;
    fn go_forwards(&mut self) -> Result<(), CalculationError>;
    fn go_backwards(&mut self) -> Result<(), CalculationError>;
    fn result(&self) -> f64;
    fn reset(&mut self);
    fn show_history(&self);
}
