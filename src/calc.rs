use std::num::{ParseFloatError, ParseIntError};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::general_operations::GeneralOperations;
use crate::logic_operations::LogicOperations;
use crate::expression_parsing::*;

macro_rules! input {
    ($t:ty) => {{
        let mut n = String::new();
        match std::io::stdin().read_line(&mut n) {
            Ok(_) => {
                let n = n.trim();
                if n.is_empty() {
                    Err(CalculationError::ParseError("Empty input".to_string()))
                } else {
                    match n.parse::<$t>() {
                        Ok(val) => Ok(val),
                        Err(e) => Err(CalculationError::ParseError(format!("Parse error: {}", e))),
                    }
                }
            }
            Err(e) => Err(CalculationError::ParseError(format!("IO error: {}", e))),
        }
    }};
}

pub struct Node {
    pub value: f64,
    parent: Option<Weak<RefCell<Node>>>,
    pub child_item: Vec<Rc<RefCell<Node>>>,
}

pub struct RustyCalculator {
    pub root: Rc<RefCell<Node>>,
    pub current: Rc<RefCell<Node>>,
    cache: Vec<Rc<RefCell<Node>>>,
}

impl RustyCalculator {
    pub fn new(rest_state: f64) -> RustyCalculator {
        let root = Rc::new(RefCell::new(Node { value: rest_state, parent: None,child_item: Vec::new(), }));
        Self { root: Rc::clone(&root), current: root, cache: Vec::new(), }
    }

    pub fn show(&self) {
        println!("{}", self.current.borrow().value);
    }

    pub fn snapshot(&mut self) {
        self.cache.push(Rc::clone(&self.root));
    }

    pub fn clear_cache(&mut self) {
        self.cache.clear();
        println!("All cached snapshots deleted.");
    }

    pub fn recover_cache(&mut self) -> Result<(), CalculationError> {
        if let Some(cached_node) = self.cache.pop() {
            self.current = cached_node;
            let mut search_node = Rc::clone(&self.current);
            loop {
                let parent_weak = search_node.borrow().parent.clone();
                if let Some(parent_weak) = parent_weak {
                    if let Some(parent_rc) = parent_weak.upgrade() {
                        search_node = parent_rc;
                    } else {
                        break;
                    }
                } else {
                    break;
                }
            }
            self.root = search_node;
            println!("Recovered to cached state with value: {}", self.current.borrow().value);
            Ok(())
        } else {
            Err(CalculationError::CannotDeleteRoot)
        }
    }

    pub fn start() -> Result<RustyCalculator, CalculationError> {
        println!("=== Rusty Calculator ===");
        println!("Commands: 'start' to begin, 'help' for help, 'quit' to exit");

        loop {
            print!("Enter command: ");
            let input: String = input!(String)?;

            match input.trim().to_lowercase().as_str() {
                "help" => Self::print_help(),
                "start" => {
                    let mut calc = RustyCalculator::new(0.0);
                    println!("Calculator started. Current value: {}", calc.current.borrow().value);
                    Self::run_calculator_loop(&mut calc)?;
                    return Ok(calc);
                }
                "quit" | "exit" => {
                    println!("Goodbye!");
                    std::process::exit(0);
                }
                _ => println!("Unknown command: '{}'. Type 'help' for options.", input),
            }
        }
    }

    fn print_help() {
        println!("\n=== Calculator Help ===");
        println!("Startup commands:");
        println!("  start - Start the calculator");
        println!("  help  - Show this help");
        println!("  quit  - Exit program");
        println!("\nCalculator operations:");
        println!("  1  - Addition");
        println!("  2  - Subtraction");
        println!("  3  - Multiplication");
        println!("  4  - Division");
        println!("  5  - Exponentiation");
        println!("  6  - Square root");
        println!("  7  - Square");
        println!("  8  - Natural logarithm");
        println!("  9  - Redo (go forwards)");
        println!("  10 - Undo (go backwards)");
        println!("  11 - Reset");
        println!("  12 - Show history");
        println!("  13 - Recover from cache");
        println!("  14 - Exit calculator");
        println!("  help - Show operations help");
        println!();
    }

    // Fixed calculator loop with proper error handling that continues operation
    fn run_calculator_loop(calc: &mut RustyCalculator) -> Result<(), CalculationError> {
        loop {
            println!("\nCurrent value: {}", calc.current.borrow().value);
            println!("Enter operation (1-14, 'help', or 'exit'):");

            let op_input: String = match input!(String) {
                Ok(v) => v,
                Err(CalculationError::ParseFloatError(_)) => {
                    println!("Input error. Please try again.");
                    continue;
                }
                Err(e) => {
                    println!("Input error: {}. Please try again.", e);
                    continue;
                }
            };
            let op_input = op_input.trim();

            // Handle special commands
            match op_input.to_lowercase().as_str() {
                "help" => {
                    Self::print_help();
                    continue;
                }
                "exit" | "quit" => break,
                _ => {}
            }

            // Try to parse as operation number
            let op_num: i32 = match op_input.parse() {
                Ok(v) => v,
                Err(_) => {
                    println!("Invalid command: '{}'. Use 1-14, 'help', or 'exit'", op_input);
                    continue;
                }
            };

            match op_num {
                1..=5 => {
                    // Operations that need a value
                    println!("Enter value:");
                    let value: f64 = match input!(f64) {
                        Ok(v) => v,
                        Err(CalculationError::ParseFloatError(e)) => {
                            println!("Invalid number: {}. Please try again.", e);
                            continue;
                        }
                        Err(e) => {
                            println!("Error: {}. Please try again.", e);
                            continue;
                        }
                    };

                    let result = match op_num {
                        1 => calc.add(value),
                        2 => calc.subtract(value),
                        3 => calc.multiply(value),
                        4 => calc.divide(value),
                        5 => calc.exp(value),
                        _ => unreachable!(),
                    };

                    // Don't break on operation errors - just show error and continue
                    if let Err(e) = result {
                        println!("Operation failed: {}. State preserved.", e);
                    }
                }
                6 => {
                    if let Err(e) = calc.square_root() {
                        println!("Square root failed: {}. State preserved.", e);
                    }
                }
                7 => {
                    if let Err(e) = calc.square() {
                        println!("Square failed: {}. State preserved.", e);
                    }
                }
                8 => {
                    if let Err(e) = calc.natural_log() {
                        println!("Natural log failed: {}. State preserved.", e);
                    }
                }
                9 => {
                    if let Err(e) = calc.go_forwards() {
                        println!("Cannot go forward: {}", e);
                    }
                }
                10 => {
                    if let Err(e) = calc.go_backwards() {
                        println!("Cannot go backward: {}", e);
                    }
                }
                11 => calc.reset(),
                12 => calc.show_history(),
                13 => {
                    if let Err(e) = calc.recover_cache() {
                        println!("Cache recovery failed: {}", e);
                    }
                }
                14 => break,
                _ => println!("Invalid option: {}. Use 1-14.", op_num),
            }
        }

        println!("Calculator session ended.");
        Ok(())
    }

    fn checked_value(_prev: f64, val: f64) -> Result<f64, CalculationError> {
        if !val.is_finite() {
            return Err(CalculationError::OutofBounds);
        }

        // Count significant digits roughly (restored from original)
        let digits = if val.abs() > 0.0 {
            val.abs().log10().floor() as i32
        } else {
            0
        };

        // f64 can only safely represent ~15 digits
        if digits > 15 {
            return Err(CalculationError::PrecisionLoss);
        }

        // Also check for extremely large values that could cause issues
        if val.abs() > f64::MAX / 2.0 {
            return Err(CalculationError::PrecisionLoss);
        }

        Ok(val)
    }
}

impl LogicOperations for RustyCalculator {
    fn add(&mut self, new_value: f64) -> Result<(), CalculationError> {
        let prev = self.current.borrow().value;
        let candidate = prev + new_value;

        let new_node = Rc::new(RefCell::new(Node {
            value: candidate,
            parent: Some(Rc::downgrade(&self.current)),
            child_item: Vec::new(),
        }));
        self.current.borrow_mut().child_item.push(Rc::clone(&new_node));
        self.current = new_node;

        match RustyCalculator::checked_value(prev, candidate) {
            Ok(valid) => {
                self.current.borrow_mut().value = valid;
                Ok(())
            }
            Err(e) => {
                let _ = self.delete();
                Err(e)
            }
        }
    }

    fn multiply(&mut self, new_value: f64) -> Result<(), CalculationError> {
        let prev = self.current.borrow().value;
        let candidate = prev * new_value;

        let new_node = Rc::new(RefCell::new(Node {
            value: candidate,
            parent: Some(Rc::downgrade(&self.current)),
            child_item: Vec::new(),
        }));
        self.current.borrow_mut().child_item.push(Rc::clone(&new_node));
        self.current = new_node;

        match RustyCalculator::checked_value(prev, candidate) {
            Ok(valid) => {
                self.current.borrow_mut().value = valid;
                Ok(())
            }
            Err(e) => {
                let _ = self.delete();
                Err(e)
            }
        }
    }

    fn divide(&mut self, divisor: f64) -> Result<(), CalculationError> {
        if divisor == 0.0 {
            return Err(CalculationError::DivisionByZero);
        }

        let previous_value = self.current.borrow().value;
        let candidate = previous_value / divisor;

        let new_node = Rc::new(RefCell::new(Node {
            value: candidate,
            parent: Some(Rc::downgrade(&self.current)),
            child_item: Vec::new(),
        }));
        self.current.borrow_mut().child_item.push(Rc::clone(&new_node));
        self.current = new_node;

        match RustyCalculator::checked_value(previous_value, candidate) {
            Ok(valid) => {
                self.current.borrow_mut().value = valid;
                Ok(())
            }
            Err(e) => {
                let _ = self.delete();
                Err(e)
            }
        }
    }

    fn subtract(&mut self, new_value: f64) -> Result<(), CalculationError> {
        let prev = self.current.borrow().value;
        let candidate = prev - new_value;

        let new_node = Rc::new(RefCell::new(Node {
            value: candidate,
            parent: Some(Rc::downgrade(&self.current)),
            child_item: Vec::new(),
        }));
        self.current.borrow_mut().child_item.push(Rc::clone(&new_node));
        self.current = new_node;

        match RustyCalculator::checked_value(prev, candidate) {
            Ok(valid) => {
                self.current.borrow_mut().value = valid;
                Ok(())
            }
            Err(e) => {
                let _ = self.delete();
                Err(e)
            }
        }
    }

    fn exp(&mut self, new_value: f64) -> Result<(), CalculationError> {
        let prev = self.current.borrow().value;
        let candidate = prev.powf(new_value);

        let new_node = Rc::new(RefCell::new(Node {
            value: candidate,
            parent: Some(Rc::downgrade(&self.current)),
            child_item: Vec::new(),
        }));
        self.current.borrow_mut().child_item.push(Rc::clone(&new_node));
        self.current = new_node;

        match RustyCalculator::checked_value(prev, candidate) {
            Ok(valid) => {
                self.current.borrow_mut().value = valid;
                Ok(())
            }
            Err(e) => {
                let _ = self.delete();
                Err(e)
            }
        }
    }

    fn square_root(&mut self) -> Result<(), CalculationError> {
        let prev = self.current.borrow().value;

        if prev < 0.0 {
            return Err(CalculationError::OutofBounds);
        }

        let candidate = prev.sqrt();

        let new_node = Rc::new(RefCell::new(Node {
            value: candidate,
            parent: Some(Rc::downgrade(&self.current)),
            child_item: Vec::new(),
        }));
        self.current.borrow_mut().child_item.push(Rc::clone(&new_node));
        self.current = new_node;

        match RustyCalculator::checked_value(prev, candidate) {
            Ok(valid) => {
                self.current.borrow_mut().value = valid;
                Ok(())
            }
            Err(e) => {
                let _ = self.delete();
                Err(e)
            }
        }
    }

    fn square(&mut self) -> Result<(), CalculationError> {
        let prev = self.current.borrow().value;
        let candidate = prev * prev;

        let new_node = Rc::new(RefCell::new(Node {
            value: candidate,
            parent: Some(Rc::downgrade(&self.current)),
            child_item: Vec::new(),
        }));
        self.current.borrow_mut().child_item.push(Rc::clone(&new_node));
        self.current = new_node;

        match RustyCalculator::checked_value(prev, candidate) {
            Ok(valid) => {
                self.current.borrow_mut().value = valid;
                Ok(())
            }
            Err(e) => {
                let _ = self.delete();
                Err(e)
            }
        }
    }

    fn natural_log(&mut self) -> Result<(), CalculationError> {
        let prev = self.current.borrow().value;

        if prev <= 0.0 {
            return Err(CalculationError::OutofBounds);
        }

        let candidate = prev.ln();

        let new_node = Rc::new(RefCell::new(Node {
            value: candidate,
            parent: Some(Rc::downgrade(&self.current)),
            child_item: Vec::new(),
        }));
        self.current.borrow_mut().child_item.push(Rc::clone(&new_node));
        self.current = new_node;

        match RustyCalculator::checked_value(prev, candidate) {
            Ok(valid) => {
                self.current.borrow_mut().value = valid;
                Ok(())
            }
            Err(e) => {
                let _ = self.delete();
                Err(e)
            }
        }
    }
}

impl GeneralOperations for RustyCalculator {
    fn input(&mut self, new_value: f64) {
        let new_node = Rc::new(RefCell::new(Node {
            value: new_value,
            parent: Some(Rc::downgrade(&self.current)),
            child_item: Vec::new(),
        }));

        self.current.borrow_mut().child_item.push(Rc::clone(&new_node));
        self.current = new_node;
    }

    fn output(&self) {
        println!("{}", self.current.borrow().value);
    }

    fn delete(&mut self) -> Result<(), CalculationError> {
        let current_node = Rc::clone(&self.current);
        let parent_weak = current_node.borrow().parent.clone();

        if let Some(parent_weak) = parent_weak {
            if let Some(parent_rc) = parent_weak.upgrade() {
                parent_rc.borrow_mut().child_item.retain(|child| !Rc::ptr_eq(child, &current_node));
                self.current = parent_rc;
                Ok(())
            } else {
                Err(CalculationError::CannotDeleteRoot)
            }
        } else {
            Err(CalculationError::CannotDeleteRoot)
        }
    }

    fn go_forwards(&mut self) -> Result<(), CalculationError> {
        let children = self.current.borrow().child_item.clone();

        if children.is_empty() {
            return Err(CalculationError::CannotGoBackwards);
        }

        println!("Choose a child to move forward to:");
        for (i, child) in children.iter().enumerate() {
            println!("{}: {}", i, child.borrow().value);
        }

        let choice: usize = match input!(usize) {
            Ok(v) => v,
            Err(CalculationError::ParseIntError(_)) => {
                return Err(CalculationError::InvalidChildIndex);
            }
            Err(_) => {
                return Err(CalculationError::InvalidChildIndex);
            }
        };

        if choice >= children.len() {
            return Err(CalculationError::InvalidChildIndex);
        }

        self.current = Rc::clone(&children[choice]);
        Ok(())
    }

    fn go_backwards(&mut self) -> Result<(), CalculationError> {
        let current_node = Rc::clone(&self.current);
        if let Some(parent_weak) = &current_node.borrow().parent {
            if let Some(parent_rc) = parent_weak.upgrade() {
                self.current = parent_rc;
                Ok(())
            } else {
                Err(CalculationError::CannotGoBackwards)
            }
        } else {
            Err(CalculationError::CannotGoBackwards)
        }
    }

    fn result(&self) -> f64 {
        self.current.borrow().value
    }

    fn reset(&mut self) {
        // Save the ENTIRE current tree state to cache, not just root
        self.snapshot(); // This now saves current position and tree

        // Create completely new calculator state
        let new_root = Rc::new(RefCell::new(Node {
            value: 0.0,
            parent: None,
            child_item: Vec::new(),
        }));
        self.root = Rc::clone(&new_root);
        self.current = new_root;

        println!("Calculator reset to 0. Full history saved to cache.");
        println!("Use option 13 to recover previous session.");
    }

    fn show_history(&self) {
        fn traverse(
            node: &Rc<RefCell<Node>>,
            current: &Rc<RefCell<Node>>,
            prefix: String,
            is_last: bool,
        ) {
            let n = node.borrow();

            print!("{}", prefix);
            if is_last {
                print!("└── ");
            } else {
                print!("├── ");
            }

            println!("{}", n.value);

            if Rc::ptr_eq(node, current) {
                println!("{}    ↑ (current)", prefix);
            }

            let new_prefix = if is_last {
                format!("{}    ", prefix)
            } else {
                format!("{}│   ", prefix)
            };

            let child_count = n.child_item.len();
            for (i, child) in n.child_item.iter().enumerate() {
                traverse(child, current, new_prefix.clone(), i == child_count - 1);
            }
        }

        println!("--- Calculator History Tree ---");
        traverse(&self.root, &self.current, "".to_string(), true);
    }
}



#[derive(Debug, Clone)]
pub enum CalculationError {
    DivisionByZero,
    ParseError(String),
    PrecisionLoss,
    CannotDeleteRoot,
    InvalidChildIndex,
    CannotGoBackwards,
    OutofBounds,
    ParseFloatError(ParseFloatError),
    ParseIntError(ParseIntError),
}

impl std::fmt::Display for CalculationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CalculationError::DivisionByZero => write!(f, "Division by zero"),
            CalculationError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CalculationError::PrecisionLoss => write!(f, "Precision loss detected"),
            CalculationError::CannotDeleteRoot => write!(f, "Cannot delete root node"),
            CalculationError::InvalidChildIndex => write!(f, "Invalid child index"),
            CalculationError::CannotGoBackwards => write!(f, "Cannot go backwards"),
            CalculationError::OutofBounds => write!(f, "Value out of bounds"),
            CalculationError::ParseFloatError(e) => write!(f, "Parse float error: {}", e),
            CalculationError::ParseIntError(e) => write!(f, "Parse int error: {}", e),
        }
    }
}
impl std::error::Error for CalculationError {}

impl From<ParseFloatError> for CalculationError {
    fn from(e: ParseFloatError) -> Self {
        CalculationError::ParseFloatError(e)
    }
}

impl From<ParseIntError> for CalculationError {
    fn from(e: ParseIntError) -> Self {
        CalculationError::ParseIntError(e)
    }
}