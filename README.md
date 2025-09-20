# RustyCalculator

A CLI calculator built with Rust, designed as a practical project to help you understand and apply key Rust language concepts.  
Rather than focusing solely on calculator functionality, this codebase uses the calculator as a vehicle to demonstrate and practice Rust's unique features in a real-world application.

## Why a Calculator?
The calculator was chosen as an approachable example to make Rust concepts easier to grasp. By implementing familiar operations (add, subtract, etc.) with Rust’s advanced features, you can directly see how ownership, error handling, traits, generics, and data structures work in practice.

## Key Rust Concepts Demonstrated

Major Rust features and idioms illustrated here include:

### 1. Ownership, Borrowing, and Lifetimes
- Uses `Rc`, `Weak`, and `RefCell` to enable shared ownership and safe mutability in a tree structure.
- Demonstrates parent-child relationships while managing lifetimes and memory safety.

### 2. Error Handling
- Custom error type `CalculationError` using Rust's `enum` and `impl Display`.
- Implements `Result<T, E>` for all fallible operations, propagating and handling errors gracefully.
- Streamlined error conversions with Rust’s trait-based system.

### 3. Pattern Matching & Control Flow
- Extensive use of `match` statements for input parsing and operation dispatch.

### 4. Traits and Polymorphism
- Calculator logic split into traits (`GeneralOperations`, `LogicOperations`) for abstraction and modularity.

### 5. Structs, Enums, and Method Organization
- Rich use of `struct` and `enum` to model calculator state, node relationships, snapshots, and error variants.

### 6. Builder Functions for Separation of Concerns and Reusability
- Builder-style functions (e.g., `Node::new`, `Node::new_root`) are used to construct and initialize objects, promoting clear separation of concerns and code reusability.

### 7. Type System and Generics
- Strong static typing and generic input parsing (`get_input<T>()`) for flexible, safe conversions.

### 8. Interior Mutability
- Uses `RefCell` for safe mutation of shared nodes, demonstrating Rust’s concurrency and mutation patterns.

### 9. Tree Data Structures
- Calculator history is a tree, supporting undo/redo and hierarchical state recovery.
- Recursive traversal and tree visualization.

### 10. CLI Interaction
- Robust command-line interface using `std::io`, generic parsing, and user-friendly error reporting.

### 11. Snapshot and Recovery Patterns
- Full calculator state can be snapshotted and restored, showing practical use of Rust’s cloning and sharing semantics.

## Getting Started

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) installed

### Build & Run

```sh
git clone https://github.com/RITIKESHDUTT/RustyCalculator.git
cd RustyCalculator
cargo build --release
cargo run
```

## Learning Outcomes

By exploring this project, you will:
- See how Rust’s ownership and borrowing help build safe, memory-efficient applications.
- Practice robust error handling, trait abstractions, and generic programming.
- Understand how to structure complex state and histories in a CLI app using Rust.

Happy Rust Learning! 🦀
