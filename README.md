# Temporal Fast Downward Planner

A high-performance temporal planning system implementing Simple Temporal Networks (STN) and advanced temporal reasoning for PDDL 2.1+ domains. Built in Rust for speed, safety, and reliability.

## 🚀 Features

### Core Planning Capabilities
- **PDDL 2.1+ Support**: Full parsing of temporal domains and problems
- **Durative Actions**: Support for actions with duration and temporal constraints
- **Temporal Operators**: `at start`, `at end`, `over all` condition and effect handling
- **Simple Temporal Networks**: Constraint-based temporal reasoning
- **A* Search**: Temporal A* search with heuristic guidance

### Advanced Features
- **Temporal Heuristics**: FF-based relaxed planning graph for temporal domains
- **Mutex Detection**: Temporal mutex analysis for action scheduling
- **Resource Constraints**: Support for numeric fluents and resource management
- **Parallel Execution**: Optional parallel processing for large-scale problems

## � Using as a Rust Library Dependency

This is the **primary way** to use the temporal planner in your Rust applications.

### Adding the Dependency

Add this to your project's `Cargo.toml`:

```toml
[dependencies]
temporal_planner = { git = "https://github.com/caelumspace/temporal-planner.git" }

# Optional: enable parallel processing
# temporal_planner = { git = "https://github.com/caelumspace/temporal-planner.git", features = ["parallel"] }

# For local development:
# temporal_planner = { path = "../temporal_planner" }
```

### Complete Integration Example

Here's a minimal working example (`src/main.rs`):

```rust
use temporal_planner::{TemporalPlanner, SearchResult};
use anyhow::Result;

fn main() -> Result<()> {
    // Create planner instance
    let mut planner = TemporalPlanner::new();

    // Solve from PDDL files
    match planner.solve_from_files("domain.pddl", "problem.pddl")? {
        SearchResult::Solution(plan) => {
            println!("✅ Found solution with {} actions", plan.actions.len());
            println!("   Total cost: {:.2}", plan.cost);
        }
        SearchResult::Failure => {
            println!("❌ No solution found");
        }
    }

    Ok(())
}
```

That's it! Your application now has full temporal planning capabilities.

### Library Features

- **Zero-copy parsing** - efficient PDDL processing
- **Type-safe API** - leverages Rust's type system
- **Error handling** - uses `Result` types for robust error management
- **Thread-safe** - can be used in multi-threaded applications
- **Memory efficient** - minimal allocations and fast execution


## 🎯 Quick Start

### Using as a Rust Library

Here's how to integrate the temporal planner into your Rust application:

```rust
use temporal_planner::{TemporalPlanner, SearchResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a planner instance
    let mut planner = TemporalPlanner::new();
    
    // Option 1: Load PDDL from files
    let result = planner.solve_from_files(
        "domain.pddl", 
        "problem.pddl"
    )?;
    
    // Option 2: Load PDDL from strings
    let domain_content = std::fs::read_to_string("domain.pddl")?;
    let problem_content = std::fs::read_to_string("problem.pddl")?;
    let result = planner.solve_from_content(&domain_content, &problem_content);
    
    // Handle the result
    match result {
        SearchResult::Solution(plan) => {
            println!("✅ Found plan with {} actions", plan.actions.len());
            println!("   Plan cost: {:.2}", plan.cost);
            
            for (i, action) in plan.actions.iter().enumerate() {
                println!("   {}. {} (time: {:.2})", 
                    i + 1, action.name, action.start_time);
            }
        }
        SearchResult::Failure => {
            println!("❌ No solution found");
        }
    }
    
    Ok(())
}
```

### Parsing Only (Without Planning)

```rust
use temporal_planner::{TemporalPlanner, TemporalTask};

fn analyze_pddl() -> Result<(), Box<dyn std::error::Error>> {
    let planner = TemporalPlanner::new();
    
    // Parse PDDL without solving
    let task = planner.load_pddl_files("domain.pddl", "problem.pddl")?;
    
    // Analyze the parsed task
    println!("📊 Domain Analysis:");
    println!("   Actions: {}", task.actions.len());
    println!("   Initial facts: {}", task.initial_state.facts.len());
    println!("   Goal conditions: {}", task.goal_conditions.len());
    
    for action in &task.actions {
        println!("   Action '{}': duration={:.1}s", 
            action.name, action.duration);
    }
    
    Ok(())
}
```

### Development and Testing

```bash
# Run the built-in test executables
cargo run --bin e2e_tests
cargo run --bin comprehensive_tests
cargo run --bin benchmark

# Run standard Rust tests
cargo test
```

## 📝 PDDL Domain Example

```pddl
(define (domain robotic-delivery)
  (:requirements :strips :typing :durative-actions)
  
  (:types robot package location)
  
  (:predicates
    (at ?r - robot ?l - location)
    (package-at ?p - package ?l - location)
    (holding ?r - robot ?p - package)
    (delivered ?p - package)
  )
  
  (:durative-action deliver
    :parameters (?r - robot ?p - package ?dest - location)
    :duration (= ?duration 2.0)
    :condition (and (at start (holding ?r ?p))
                    (at start (at ?r ?dest)))
    :effect (and (at end (delivered ?p))
                 (at end (not (holding ?r ?p))))
  )
)
```

## 🏗️ Architecture

### Core Components

```
temporal_planner/
├── src/
│   ├── temporal_task.rs     # PDDL parsing and task representation
│   ├── search.rs            # Temporal A* search implementation
│   ├── state_space.rs       # State space and action application
│   ├── heuristics.rs        # Temporal heuristics (FF, admissible)
│   ├── scheduler.rs         # Simple Temporal Network management
│   └── lib.rs              # Public API and main interfaces
├── tests/                   # End-to-end test suite
└── docs/                   # Documentation and examples
```

### Key Data Structures

- **`TemporalTask`**: Represents a complete planning problem
- **`TemporalAction`**: Durative action with temporal constraints
- **`TemporalState`**: State with scheduled effects and time
- **`SimpleTemporalNetwork`**: Manages temporal constraints

## 🧪 Testing & Validation

### Test Suite Overview
The project includes a comprehensive end-to-end test suite that validates:

- **PDDL Parsing**: Domain and problem file parsing accuracy
- **Temporal Semantics**: Durative action and temporal operator handling
- **Planning Integration**: Complete pipeline from parsing to plan generation
- **Performance**: Parsing speed and memory usage benchmarks
- **Error Handling**: Graceful handling of invalid inputs

### Test Domains
- **Simple Robot**: Basic navigation and package delivery (4 actions)
- **Blocks World**: Classic planning with temporal extensions (4 actions)  
- **Factory Automation**: Complex manufacturing with resource constraints (4 actions)

### Running Tests
```bash
# Quick functional validation
cargo run --bin e2e_tests

# Detailed analysis with JSON report
cargo run --bin comprehensive_tests

# Performance benchmarking
cargo run --bin benchmark

# Standard unit tests
cargo test
```

### Test Results
Current test results show:
- ✅ **100% Success Rate** across all test scenarios
- ✅ **12 Actions Parsed** correctly with temporal constraints
- ✅ **~25ms Average** parsing time per domain
- ✅ **40+ parses/second** throughput

## 📊 Performance

### Benchmarks
Based on current test suite results:

| Domain Type | Parse Time | Actions | Throughput |
|-------------|-----------|---------|------------|
| Simple Robot | ~27ms | 4 (1 durative) | 36.8/sec |
| Blocks World | ~25ms | 4 (1 durative) | 39.7/sec |
| Factory Automation | ~25ms | 4 (0 durative) | 39.7/sec |

### Memory Usage
- **Linear scaling** up to 1000+ task instances
- **~19ms per task** creation time
- **Efficient memory management** with Rust's ownership system

## 🛠️ Development

### Building
```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# With parallel processing feature
cargo build --features parallel
```

### Code Style
```bash
# Format code
cargo fmt

# Lint code
cargo clippy

# Check without building
cargo check
```

### Adding New Domains
1. Create PDDL files in `tests/fixtures/domains/` and `tests/fixtures/problems/`
2. Add test cases in `tests/integration_tests.rs`
3. Update the comprehensive test suite in `src/bin/comprehensive_tests.rs`

## 🤝 Contributing

### Guidelines
1. **Code Quality**: Ensure all tests pass and maintain >95% coverage
2. **Documentation**: Update README and inline docs for new features
3. **Performance**: Benchmark new features and avoid regressions
4. **Testing**: Add comprehensive tests for new functionality

### Development Workflow
```bash
# 1. Create feature branch
git checkout -b feature/new-heuristic

# 2. Implement changes with tests
cargo test

# 3. Run full test suite
cargo run --bin comprehensive_tests

# 4. Performance validation
cargo run --bin benchmark

# 5. Submit pull request
```

## 📚 API Documentation

### Core API Usage

The main entry point for external Rust applications:

```rust
use temporal_planner::{TemporalPlanner, TemporalTask, SearchResult};

// Create planner instance
let mut planner = TemporalPlanner::new();

// Get planner information
let info = planner.get_info();
println!("Planner version: {}", info.version);
println!("Search algorithm: {}", info.search_algorithm);
```

### Loading PDDL Content

```rust
// From file paths
let task = planner.load_pddl_files("domain.pddl", "problem.pddl")?;

// From string content
let task = planner.load_pddl_content(&domain_str, &problem_str);

// Access parsed information
println!("Actions: {}", task.actions.len());
for action in &task.actions {
    println!("Action: {} (duration: {})", action.name, action.duration);
}
```

### Planning and Solution Handling

```rust
// Solve with pre-loaded task
let result = planner.solve(&task);

// Complete pipeline methods
let result = planner.solve_from_files("domain.pddl", "problem.pddl")?;
let result = planner.solve_from_content(&domain_str, &problem_str);

// Handle results
match result {
    SearchResult::Solution(plan) => {
        println!("Found plan with {} actions", plan.actions.len());
        println!("Plan cost: {:.2}", plan.cost);
        
        // Access individual actions
        for action in &plan.actions {
            println!("Action: {} at time {:.2}", action.name, action.start_time);
        }
    }
    SearchResult::Failure => {
        println!("No solution found");
    }
}
```

### Working with Temporal Actions

```rust
use temporal_planner::{TemporalAction, Condition, Effect};

// Actions have temporal structure
for action in &task.actions {
    println!("Action: {}", action.name);
    println!("  Duration: {:.2}s", action.duration);
    println!("  Start conditions: {}", action.conditions_start.len());
    println!("  Over-all conditions: {}", action.conditions_over_all.len());
    println!("  End conditions: {}", action.conditions_end.len());
    println!("  Start effects: {}", action.effects_start.len());
    println!("  End effects: {}", action.effects_end.len());
}
```

### Error Handling

```rust
use anyhow::Result;

fn my_planning_function() -> Result<()> {
    let mut planner = TemporalPlanner::new();
    
    // Methods return Results for proper error handling
    match planner.solve_from_files("domain.pddl", "problem.pddl") {
        Ok(SearchResult::Solution(plan)) => {
            println!("Success! Plan has {} actions", plan.actions.len());
        }
        Ok(SearchResult::Failure) => {
            println!("Planning failed - no solution exists");
        }
        Err(e) => {
            eprintln!("Error during planning: {}", e);
            return Err(e);
        }
    }
    
    Ok(())
}
```

## 🔧 Configuration

### Cargo Features
- `parallel`: Enable parallel processing with rayon
- `default`: Standard features for most use cases

### Environment Variables
- `RUST_LOG`: Set logging level (debug, info, warn, error)
- `RUST_BACKTRACE`: Enable backtrace on panic

## 📄 License

This project is licensed under the MIT OR Apache-2.0 license.

## 🙏 Acknowledgments

- **Fast Downward**: Inspiration for the planning framework
- **PDDL Community**: Standards and domain examples
- **Rust Community**: Excellent ecosystem and tooling

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/caelumspace/temporal-planner/issues)
- **Discussions**: [GitHub Discussions](https://github.com/caelumspace/temporal-planner/discussions)
- **Documentation**: See `docs/` directory for detailed guides

---

**Built with ❤️ in Rust for high-performance temporal planning**