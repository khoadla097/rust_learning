# Rust Learning Journey 🦀

This repository tracks my learning progress through the interactive book [The Rust Programming Language](https://rust-book.cs.brown.edu/) (Brown University Edition).

---

## 📂 Repository Structure

The project is organized as a **Cargo Workspace**, with lessons organized chapter by chapter:

```text
rust_learning/
├── Cargo.toml                              # Root workspace configuration
├── README.md                               # Learning roadmap & progress checklist
├── .gitignore                              # Git ignore rules for binaries and target
├── ch01_getting_started/
│   ├── hello_world/                        # Chapter 1.2: Direct compilation with rustc
│   │   └── main.rs
│   └── hello_cargo/                        # Chapter 1.3: Crate initialized with Cargo
│       ├── Cargo.toml
│       └── src/main.rs
├── ch02_guessing_game/                     # Chapter 2: Complete guessing game project
│   ├── Cargo.toml
│   └── src/main.rs
├── ch03_common_concepts/
│   ├── variables/                          # Chapter 3.1 & 3.2: Variables, Mutability & Data Types
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   ├── functions/                          # Chapter 3.3: Functions
│   │   ├── Cargo.toml
│   │   └── src/main.rs
│   └── control_flow/                       # Chapter 3.5: Control Flow
│       ├── Cargo.toml
│       └── src/main.rs
└── ch04_owner_ship/
    ├── ownership/                          # Chapter 4.1: Ownership fundamentals
    │   ├── Cargo.toml
    │   └── src/main.rs
    ├── reference/                          # Chapter 4.2: References and Borrowing
    │   ├── Cargo.toml
    │   └── src/main.rs
    └── slice/                              # Chapter 4.4: The Slice Type
        ├── Cargo.toml
        └── src/main.rs
```

---

## ⚡ Useful Commands (Cargo Workspace)

Since this repository is managed using a Cargo Workspace, you can run commands directly from the root directory:

- **Check syntax across the entire workspace:**
  ```bash
  cargo check --workspace
  ```

- **Run a specific exercise/project by package name:**
  ```bash
  cargo run -p guessing_game
  cargo run -p variables
  cargo run -p functions
  cargo run -p control_flow
  cargo run -p ownership
  cargo run -p reference
  cargo run -p slice
  cargo run -p hello_cargo
  ```

- **Add a new exercise to a chapter (e.g., adding `control_flow` to Chapter 3):**
  ```bash
  cargo new ch03_common_concepts/control_flow
  ```
  Then declare `"ch03_common_concepts/control_flow"` in the `members` list in the root `Cargo.toml`.

- **Run a standalone rustc script (Hello World):**
  ```bash
  cd ch01_getting_started/hello_world
  rustc main.rs
  ./main
  ```

---

## 🗺️ Roadmap & Progress Checklist (The Book)

- [x] **Chapter 1: Getting Started** (`ch01_getting_started`)
  - [x] 1.1 Installation
  - [x] 1.2 Hello, World! (`hello_world`)
  - [x] 1.3 Hello, Cargo! (`hello_cargo`)
- [x] **Chapter 2: Programming a Guessing Game** (`ch02_guessing_game`)
  - [x] 2.0 Programming a Guessing Game (`guessing_game`)
- [x] **Chapter 3: Common Programming Concepts** (`ch03_common_concepts`)
  - [x] 3.1 Variables and Mutability (`variables`)
  - [x] 3.2 Data Types (`variables`)
  - [x] 3.3 Functions (`functions`)
  - [x] 3.4 Comments
  - [x] 3.5 Control Flow (`control_flow`)
- [x] **Chapter 4: Understanding Ownership** (`ch04_owner_ship`)
  - [x] 4.1 What is Ownership? (`ownership`)
  - [x] 4.2 References and Borrowing (`reference`)
  - [x] 4.3 Fixing Ownership Errors
  - [x] 4.4 The Slice Type (`slice`)
  - [x] 4.5 Ownership Recap
- [ ] **Chapter 5: Using Structs to Structure Related Data** (`ch05_structs`)
  - [ ] 5.1 Defining and Instantiating Structs
  - [ ] 5.2 An Example Program Using Structs
  - [ ] 5.3 Methods
- [ ] **Chapter 6: Enums and Pattern Matching** (`ch06_enums`)
  - [ ] 6.1 Defining an Enum
  - [ ] 6.2 The `match` Control Flow Construct
  - [ ] 6.3 Concise Control Flow with `if let` and `let...else`
  - [ ] 6.4 Ownership Inventory #1
- [ ] **Chapter 7: Packages, Crates, and Modules** (`ch07_modules`)
  - [ ] 7.1 Packages and Crates
  - [ ] 7.2 Control Scope and Privacy with Modules
  - [ ] 7.3 Paths for Referring to an Item in the Module Tree
  - [ ] 7.4 Bringing Paths Into Scope with the `use` Keyword
  - [ ] 7.5 Separating Modules into Different Files
- [ ] **Chapter 8: Common Collections** (`ch08_collections`)
  - [ ] 8.1 Storing Lists of Values with Vectors
  - [ ] 8.2 Storing UTF-8 Encoded Text with Strings
  - [ ] 8.3 Storing Keys with Associated Values in Hash Maps
  - [ ] 8.4 Ownership Inventory #2
- [ ] **Chapter 9: Error Handling** (`ch09_error_handling`)
  - [ ] 9.1 Unrecoverable Errors with `panic!`
  - [ ] 9.2 Recoverable Errors with `Result`
  - [ ] 9.3 To `panic!` or Not to `panic!`
- [ ] **Chapter 10: Generic Types, Traits, and Lifetimes** (`ch10_generics_traits_lifetimes`)
  - [ ] 10.1 Generic Data Types
  - [ ] 10.2 Defining Shared Behavior with Traits
  - [ ] 10.3 Validating References with Lifetimes
  - [ ] 10.4 Ownership Inventory #3
- [ ] **Chapter 11: Writing Automated Tests** (`ch11_testing`)
  - [ ] 11.1 How to Write Tests
  - [ ] 11.2 Controlling How Tests Are Run
  - [ ] 11.3 Test Organization
- [ ] **Chapter 12: An I/O Project: Building a Command Line Program** (`ch12_minigrep`)
  - [ ] 12.1 Accepting Command Line Arguments
  - [ ] 12.2 Reading a File
  - [ ] 12.3 Refactoring to Improve Modularity and Error Handling
  - [ ] 12.4 Adding Functionality with Test Driven Development
  - [ ] 12.5 Working with Environment Variables
  - [ ] 12.6 Redirecting Errors to Standard Error
- [ ] **Chapter 13: Functional Language Features: Iterators and Closures** (`ch13_functional_features`)
  - [ ] 13.1 Closures
  - [ ] 13.2 Processing a Series of Items with Iterators
  - [ ] 13.3 Improving Our I/O Project
  - [ ] 13.4 Performance in Loops vs. Iterators
- [ ] **Chapter 14: More About Cargo and Crates.io** (`ch14_cargo_crates_io`)
  - [ ] 14.1 Customizing Builds with Release Profiles
  - [ ] 14.2 Publishing a Crate to Crates.io
  - [ ] 14.3 Cargo Workspaces
  - [ ] 14.4 Installing Binaries with `cargo install`
  - [ ] 14.5 Extending Cargo with Custom Commands
- [ ] **Chapter 15: Smart Pointers** (`ch15_smart_pointers`)
  - [ ] 15.1 Using `Box<T>` to Point to Data on the Heap
  - [ ] 15.2 Treating Smart Pointers Like Regular References
  - [ ] 15.3 Running Code on Cleanup with the `Drop` Trait
  - [ ] 15.4 `Rc<T>`, the Reference-Counted Smart Pointer
  - [ ] 15.5 `RefCell<T>` and the Interior Mutability Pattern
  - [ ] 15.6 Reference Cycles Can Leak Memory
- [ ] **Chapter 16: Fearless Concurrency** (`ch16_concurrency`)
  - [ ] 16.1 Using Threads to Run Code Simultaneously
  - [ ] 16.2 Transfer Data Between Threads with Message Passing
  - [ ] 16.3 Shared-State Concurrency
  - [ ] 16.4 Extensible Concurrency with Send and Sync
- [ ] **Chapter 17: Fundamentals of Asynchronous Programming: Async, Await, Futures, and Streams** (`ch17_async_await`)
  - [ ] 17.1 Futures and the Async Syntax
  - [ ] 17.2 Applying Concurrency with Async
  - [ ] 17.3 Working With Any Number of Futures
  - [ ] 17.4 Streams: Futures in Sequence
  - [ ] 17.5 A Closer Look at the Traits for Async
  - [ ] 17.6 Futures, Tasks, and Threads
- [ ] **Chapter 18: Object-Oriented Programming Features** (`ch18_oop_features`)
  - [ ] 18.1 Characteristics of Object-Oriented Languages
  - [ ] 18.2 Using Trait Objects to Abstract over Shared Behavior
  - [ ] 18.3 Implementing an Object-Oriented Design Pattern
  - [ ] 18.4 Ownership Inventory #4
  - [ ] 18.5 Design Trade-offs
- [ ] **Chapter 19: Patterns and Matching** (`ch19_patterns`)
  - [ ] 19.1 All the Places Patterns Can Be Used
  - [ ] 19.2 Refutability: Whether a Pattern Might Fail to Match
  - [ ] 19.3 Pattern Syntax
- [ ] **Chapter 20: Advanced Features** (`ch20_advanced_features`)
  - [ ] 20.1 Unsafe Rust
  - [ ] 20.2 Advanced Traits
  - [ ] 20.3 Advanced Types
  - [ ] 20.4 Advanced Functions and Closures
  - [ ] 20.5 Macros
- [ ] **Chapter 21: Final Project: Building a Multithreaded Web Server** (`ch21_web_server`)
  - [ ] 21.1 Building a Single-Threaded Web Server
  - [ ] 21.2 From Single-Threaded to Multithreaded Server
  - [ ] 21.3 Graceful Shutdown and Cleanup
