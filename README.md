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

## 📦 Installation

### Prerequisites
- Rust 1.70+ 
- Cargo (comes with Rust)

### Build from Source
```bash
git clone https://github.com/caelumspace/temporal-planner.git
cd temporal-planner
cargo build --release
```

### Dependencies
- `serde` - Serialization for data structures
- `regex` - PDDL parsing and pattern matching
- `nalgebra` - Linear algebra for temporal constraints
- `clap` - Command-line interface
- `anyhow` - Error handling

## 🎯 Quick Start

### Running the PDDL Parser Demo
```bash
cargo run --bin pddl_parser_demo
```

### Running End-to-End Tests
```bash
# Basic functional tests
cargo run --bin e2e_tests

# Comprehensive test suite with metrics
cargo run --bin comprehensive_tests

# Performance benchmarks
cargo run --bin benchmark
```

### Standard Rust Tests
```bash
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

### Basic Usage
```rust
use temporal_planner::TemporalTask;

// Parse PDDL domain and problem
let task = TemporalTask::from_pddl(domain_content, problem_content);

// Access parsed information
println!("Actions: {}", task.actions.len());
for action in &task.actions {
    println!("Action: {} (duration: {})", action.name, action.duration);
}
```

### Planning Integration
```rust
use temporal_planner::{TemporalAStarSearch, TemporalSearchEngine};

let mut search_engine = TemporalAStarSearch::new();
let result = search_engine.search(&task);

match result {
    SearchResult::Solution(plan) => {
        println!("Found plan with {} actions", plan.actions.len());
    }
    SearchResult::Failure => {
        println!("No solution found");
    }
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