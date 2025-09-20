use std::num::{ParseFloatError, ParseIntError};
use std::rc::{Rc, Weak};
use std::cell::RefCell;
use crate::general_operations::GeneralOperations;
use crate::logic_operations::LogicOperations;

fn get_input<T>() -> Result<T, CalculationError>  where T: std::str::FromStr, T::Err: std::fmt::Display, {
    let mut input = String::new();
    match std::io::stdin().read_line(&mut input) {
        Ok(_) => { let input = input.trim(); if input.is_empty() {
                Err(CalculationError::ParseError("Empty input".to_string()))
            } else { match input.parse::<T>() {
                    Ok(val) => Ok(val),
                    Err(e) => Err(CalculationError::ParseError(format!("Parse error: {}", e))), }
            }
        }
        Err(e) => Err(CalculationError::ParseError(format!("IO error: {}", e))), }
}

// Complete snapshot of calculator state for proper recovery
#[derive(Clone)]
struct CalculatorSnapshot {
    root: Rc<RefCell<Node>>,
    current: Rc<RefCell<Node>>,
    history: Vec<Rc<RefCell<Node>>>,
    history_index: usize,
}

pub struct Node {
    value: f64,
    parent: Option<Weak<RefCell<Node>>>,
    child_item: Vec<Rc<RefCell<Node>>>,
    last_op: Option<String>,
}

impl Node {
    fn new(value: f64, parent: Option<&Rc<RefCell<Node>>>, op: Option<String>) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(Self { value, last_op: op, parent: parent.map(|p| Rc::downgrade(p)), child_item: Vec::new(), }))
    }

    // Convenience method for root nodes (maintains existing API)
    fn new_root(value: f64) -> Rc<RefCell<Self>> {
        Self::new(value, None, None)
    }
}

pub struct RustyCalculator {
    root: Rc<RefCell<Node>>,
    current: Rc<RefCell<Node>>,
    history: Vec<Rc<RefCell<Node>>>,
    history_index: usize,
    snapshots: Vec<CalculatorSnapshot>,  // Store complete calculator states
}

impl RustyCalculator {
    pub fn new(rest_state: f64) -> RustyCalculator {
        let root = Node::new_root(rest_state);
        Self {
            root: Rc::clone(&root),
            current: Rc::clone(&root),
            history: vec![Rc::clone(&root)],
            history_index: 0,
            snapshots: Vec::new(),
        }
    }

    fn insert_node(&mut self, value: f64, op: Option<String>) -> Rc<RefCell<Node>> {
        let new_node = Node::new(value, Some(&self.current), op);
        self.current.borrow_mut().child_item.push(Rc::clone(&new_node));

        // When creating new nodes, truncate history after current position and add new node
        self.history.truncate(self.history_index + 1);
        self.history.push(Rc::clone(&new_node));
        self.history_index = self.history.len() - 1; // always point to last node
        self.current = Rc::clone(&new_node);

        new_node
    }

    // Apply operation with automatic last_op tracking
    fn apply_op<F>(&mut self, op_fn: F, op_label: &str) -> Result<(), CalculationError>
    where F: FnOnce(f64) -> f64, {
        let prev = self.current.borrow().value;
        let candidate = op_fn(prev);

        // Insert node with operation label
        self.insert_node(candidate, Some(op_label.to_string()));

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

    pub fn show(&self) {
        println!("{}", self.current.borrow().value);
    }

    // Store complete calculator state including root, current, and full history
    pub fn snapshot(&mut self) {
        let snapshot = CalculatorSnapshot {
            root: Rc::clone(&self.root),
            current: Rc::clone(&self.current),
            history: self.history.clone(),
            history_index: self.history_index,
        };
        self.snapshots.push(snapshot);
    }

    pub fn clear_cache(&mut self) {
        self.snapshots.clear();
        println!("All cached snapshots deleted.");
    }

    pub fn recover_cache(&mut self) -> Result<(), CalculationError> {
        if let Some(snapshot) = self.snapshots.pop() {
            // Restore complete calculator state from snapshot
            self.root = snapshot.root;
            self.current = snapshot.current;
            self.history = snapshot.history;
            self.history_index = snapshot.history_index;

            println!("Recovered to cached state with value: {}", self.current.borrow().value);
            Ok(())
        } else {
            Err(CalculationError::CannotDeleteRoot)
        }
    }

    // Unified value validation - combines all boundary checks
    fn checked_value(_prev: f64, val: f64) -> Result<f64, CalculationError> {
        if !val.is_finite() {
            return Err(CalculationError::OutOfBounds);
        }
        let digits = if val.abs() > 0.0 { val.abs().log10().floor() as i32 } else { 0 };
        if digits > 15 || val.abs() > f64::MAX / 2.0 {
            return Err(CalculationError::PrecisionLoss);
        }
        Ok(val)
    }

    pub fn start() -> Result<RustyCalculator, CalculationError> {
        println!("=== Rusty Calculator ===");
        println!("Commands: 'start' to begin, 'help' for help, 'quit' to exit");

        loop {
            print!("Enter command: ");
            let input: String = get_input::<String>()?;

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

    // Centralized input handling for operations that require values
    fn get_operation_value() -> Result<f64, CalculationError> {
        println!("Enter value:");
        get_input::<f64>()
    }

    // Centralized error reporting for operations
    fn handle_operation_result(result: Result<(), CalculationError>, operation: &str) {
        if let Err(e) = result {
            println!("{} failed: {}. State preserved.", operation, e);
        }
    }

    fn run_calculator_loop(calc: &mut RustyCalculator) -> Result<(), CalculationError> {
        loop {
            println!("\nCurrent value: {}", calc.current.borrow().value);
            println!("Enter operation (1-14, 'help', or 'exit'):");

            let op_input: String = match get_input::<String>() {
                Ok(v) => v,
                Err(_) => { println!("Input error. Please try again."); continue; }
            };
            let op_input = op_input.trim();

            match op_input.to_lowercase().as_str() {
                "help" => { Self::print_help(); continue; }
                "exit" | "quit" => break,
                _ => {}
            }

            let op_num: i32 = match op_input.parse() {
                Ok(v) => v,
                Err(_) => { println!("Invalid command: '{}'. Use 1-14, 'help', or 'exit'", op_input); continue; }
            };

            match op_num {
                // Operations requiring input values
                1..=5 => {
                    match Self::get_operation_value() {
                        Ok(value) => {
                            let result = match op_num {
                                1 => calc.add(value),
                                2 => calc.subtract(value),
                                3 => calc.multiply(value),
                                4 => calc.divide(value),
                                5 => calc.exp(value),
                                _ => unreachable!(),
                            };
                            let op_name = match op_num {
                                1 => "Addition",
                                2 => "Subtraction",
                                3 => "Multiplication",
                                4 => "Division",
                                5 => "Exponentiation",
                                _ => unreachable!(),
                            };
                            Self::handle_operation_result(result, op_name);
                        }
                        Err(_) => { println!("Invalid number. Try again."); continue; }
                    }
                }
                // Single-value operations
                6 => Self::handle_operation_result(calc.square_root(), "Square root"),
                7 => Self::handle_operation_result(calc.square(), "Square"),
                8 => Self::handle_operation_result(calc.natural_log(), "Natural log"),
                // Navigation operations
                9 => Self::handle_operation_result(calc.go_forwards(), "Redo"),
                10 => Self::handle_operation_result(calc.go_backwards(), "Undo"),
                // Utility operations
                11 => calc.reset(),
                12 => calc.show_history(),
                13 => Self::handle_operation_result(calc.recover_cache(), "Cache recovery"),
                14 => break,
                _ => println!("Invalid option: {}. Use 1-14.", op_num),
            }
        }

        println!("Calculator session ended.");
        Ok(())
    }

    fn print_help() {
        println!("\n=== Calculator Help ===");
        let startup_cmds: &[(&str, &str)] = &[
            ("start", "Start the calculator"),
            ("help", "Show this help"),
            ("quit", "Exit program"),
        ];
        let calc_ops: &[(&str, &str)] = &[("1", "Addition"), ("2", "Subtraction"), ("3", "Multiplication"), ("4", "Division"),
            ("5", "Exponentiation"), ("6", "Square root"), ("7", "Square"), ("8", "Natural logarithm"), ("9", "Redo (go forwards)"),
            ("10", "Undo (go backwards)"), ("11", "Reset"), ("12", "Show history"), ("13", "Recover from cache"), ("14", "Exit calculator"),
            ("help", "Show operations help"),
        ];
        let sections: &[(&str, &[(&str, &str)])] = &[
            ("Startup commands", startup_cmds),
            ("Calculator operations", calc_ops),
        ];
        for (title, commands) in sections {
            println!("{}:", title);
            for (cmd, desc) in *commands {
                println!("  {:<5} - {}", cmd, desc);
            }
            println!();
        }
    }
}

impl LogicOperations for RustyCalculator {
    fn add(&mut self, val: f64) -> Result<(), CalculationError> {
        self.apply_op(|prev| prev + val, "+")
    }
    fn subtract(&mut self, val: f64) -> Result<(), CalculationError> {
        self.apply_op(|prev| prev - val, "-")
    }
    fn multiply(&mut self, val: f64) -> Result<(), CalculationError> {
        self.apply_op(|prev| prev * val, "*")
    }
    fn divide(&mut self, val: f64) -> Result<(), CalculationError> {
        if val == 0.0 { return Err(CalculationError::DivisionByZero); }
        self.apply_op(|prev| prev / val, "/")
    }
    fn exp(&mut self, val: f64) -> Result<(), CalculationError> {
        self.apply_op(|prev| prev.powf(val), "^")
    }
    fn square(&mut self) -> Result<(), CalculationError> {
        self.apply_op(|prev| prev * prev, "sqr")
    }
    fn square_root(&mut self) -> Result<(), CalculationError> {
        if self.current.borrow().value < 0.0 { return Err(CalculationError::OutOfBounds); }
        self.apply_op(|prev| prev.sqrt(), "√")
    }
    fn natural_log(&mut self) -> Result<(), CalculationError> {
        if self.current.borrow().value <= 0.0 { return Err(CalculationError::OutOfBounds); }
        self.apply_op(|prev| prev.ln(), "ln")
    }
}

impl GeneralOperations for RustyCalculator {
    fn input(&mut self, val: f64) {
        // For direct input, no operation associated
        self.insert_node(val, None);
    }

    fn output(&self) {
        println!("{}", self.current.borrow().value);
    }

    fn delete(&mut self) -> Result<(), CalculationError> {
        let current_node = Rc::clone(&self.current);
        if let Some(parent_weak) = &current_node.borrow().parent {
            if let Some(parent_rc) = parent_weak.upgrade() {
                parent_rc.borrow_mut().child_item.retain(|child| !Rc::ptr_eq(child, &current_node));
                self.current = parent_rc;
                Ok(())
            } else { Err(CalculationError::CannotDeleteRoot) }
        } else { Err(CalculationError::CannotDeleteRoot) }
    }

    fn go_backwards(&mut self) -> Result<(), CalculationError> {
        if self.history_index == 0 { return Err(CalculationError::CannotGoBackwards); }
        self.history_index -= 1;
        self.current = Rc::clone(&self.history[self.history_index]);
        Ok(())
    }

    fn go_forwards(&mut self) -> Result<(), CalculationError> {
        // Fixed: Use correct error type for forward navigation
        if self.history_index + 1 >= self.history.len() {
            return Err(CalculationError::CannotGoForwards); }
        self.history_index += 1;
        self.current = Rc::clone(&self.history[self.history_index]);
        Ok(())
    }

    fn result(&self) -> f64 {
        self.current.borrow().value
    }

    fn reset(&mut self) {
        self.snapshot();
        let new_root = Node::new_root(0.0);
        self.root = Rc::clone(&new_root);
        self.current = Rc::clone(&new_root);
        self.history.clear();
        self.history.push(Rc::clone(&new_root));
        self.history_index = 0;
        println!("Calculator reset to 0. Full history saved to snapshots.");
    }

    fn show_history(&self) {
        fn traverse(node: &Rc<RefCell<Node>>, current: &Rc<RefCell<Node>>, prefix: String, is_last: bool) {
            let n = node.borrow();
            print!("{}", prefix);
            print!("{}", if is_last { "└── " } else { "├── " });
            print!("{}", n.value);
            if let Some(op) = &n.last_op {
                print!(" | {}", op);
            }
            println!();
            if Rc::ptr_eq(node, current) {
                println!("{}    ↑ (current)", prefix);
            }

            let new_prefix = if is_last { format!("{}    ", prefix) } else { format!("{}│   ", prefix) };
            let count = n.child_item.len();
            for (i, child) in n.child_item.iter().enumerate() {
                traverse(child, current, new_prefix.clone(), i == count - 1);
            }
        }

        println!("--- Calculator History Tree ---");
        traverse(&self.root, &self.current, "".to_string(), true);
    }
}

// Simplified error enum - removed redundant ParseFloatError and ParseIntError variants
// ParseError(String) handles all parsing errors uniformly
#[derive(Debug, Clone)]
pub enum CalculationError {
    DivisionByZero,
    ParseError(String),            // Unified parsing error handling
    PrecisionLoss,
    CannotDeleteRoot,
    InvalidChildIndex,
    CannotGoBackwards,
    CannotGoForwards,              // Added missing forward navigation error
    OutOfBounds,
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
            CalculationError::CannotGoForwards => write!(f, "Cannot go forwards"),
            CalculationError::OutOfBounds => write!(f, "Value out of bounds"),
        }
    }
}

impl std::error::Error for CalculationError {}
// Simplified From implementations - all parse errors go through ParseError(String)
impl From<ParseFloatError> for CalculationError {
    fn from(e: ParseFloatError) -> Self {
        CalculationError::ParseError(format!("Float parse error: {}", e))
    }
}
impl From<ParseIntError> for CalculationError {
    fn from(e: ParseIntError) -> Self {
        CalculationError::ParseError(format!("Integer parse error: {}", e))
    }
}
