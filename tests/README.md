# Temporal Planner End-to-End Test Suite

This directory contains comprehensive end-to-end tests for the temporal planning system, covering PDDL parsing, domain validation, and planning integration.

## Test Structure

```
tests/
├── fixtures/
│   ├── domains/          # PDDL domain files
│   └── problems/         # PDDL problem files
├── integration_tests.rs  # Rust integration tests
└── README.md            # This file
```

## Available Test Binaries

### 1. Basic End-to-End Tests
```bash
cargo run --bin e2e_tests
```
Runs basic functional tests with simple output.

### 2. Comprehensive Test Suite
```bash
cargo run --bin comprehensive_tests
```
Runs detailed tests with metrics collection and JSON report generation.

### 3. Performance Benchmarks
```bash
cargo run --bin benchmark
```
Runs performance benchmarks for parsing and memory usage.

### 4. PDDL Parser Demo
```bash
cargo run --bin pddl_parser_demo
```
Demonstrates the PDDL parser with example domain and problem.

## Running Standard Rust Tests

### Integration Tests
```bash
cargo test --test integration_tests
```

### All Tests
```bash
cargo test
```

### Specific Test Functions
```bash
cargo test test_simple_robot_domain_parsing
cargo test test_blocks_world_domain_parsing
cargo test test_factory_automation_complex_parsing
```

## Test Domains and Problems

### 1. Simple Robot Domain (`simple_robot.pddl`)
- **Purpose**: Basic robot navigation and package delivery
- **Features**: Regular actions, one durative action
- **Problem**: Deliver packages from depot to destinations
- **Expected**: 4 actions (move, pick-up, drop, deliver)

### 2. Blocks World Domain (`blocks_world.pddl`)
- **Purpose**: Classic planning domain with temporal extensions
- **Features**: Block manipulation with slow stacking
- **Problem**: Stack blocks in specific order
- **Expected**: 4 actions including durative `stack-slow`

### 3. Factory Automation Domain (`factory_automation.pddl`)
- **Purpose**: Complex manufacturing scenario
- **Features**: Multiple durative actions, resource constraints
- **Problem**: Process ingredients and produce products
- **Expected**: 4 actions with variable durations

## Test Output and Reports

### Console Output
All test binaries provide colored console output with:
- ✅ Passed tests
- ❌ Failed tests  
- ⚠️ Skipped tests
- 📊 Performance metrics

### JSON Reports
The `comprehensive_tests` binary generates `test_results.json` with:
- Detailed test results
- Performance metrics
- Parsing statistics
- Error messages

Example report structure:
```json
{
  "timestamp": "1234567890",
  "total_tests": 5,
  "passed_tests": 4,
  "failed_tests": 1,
  "test_results": [...],
  "summary": {
    "success_rate": 80.0,
    "total_actions_parsed": 12,
    "average_parse_time_ms": 2.5,
    "domains_tested": ["Simple Robot", "Blocks World"],
    "planning_successful": false
  }
}
```

## Expected Test Results

Based on current implementation:

| Test | Status | Expected Actions | Notes |
|------|--------|------------------|-------|
| Simple Robot | ✅ Pass | 4 | Includes durative `deliver` |
| Blocks World | ✅ Pass | 4 | Includes durative `stack-slow` |
| Factory Automation | ✅ Pass | 4 | Complex domain with numeric fluents |
| Temporal Properties | ✅ Pass | 1 | Tests temporal operator parsing |
| Planning Integration | ⚠️ Skip | N/A | Requires complete search implementation |

## Adding New Test Cases

### 1. Create PDDL Files
Add domain and problem files to appropriate `fixtures/` subdirectories.

### 2. Add Test Function
```rust
#[test]
fn test_new_domain() {
    let domain_path = "tests/fixtures/domains/new_domain.pddl";
    let problem_path = "tests/fixtures/problems/new_problem.pddl";
    // ... test implementation
}
```

### 3. Update Test Runner
Add the new test to the comprehensive test suite in `comprehensive_tests.rs`.

## Debugging Test Failures

### 1. Verbose Output
Run tests with verbose output to see detailed parsing information:
```bash
cargo run --bin e2e_tests 2>&1 | tee test_output.log
```

### 2. Individual Domain Testing
Test specific domains in isolation:
```bash
cargo run --bin pddl_parser_demo
```

### 3. Check Test Reports
Review the generated `test_results.json` for detailed failure information.

## Performance Expectations

Typical performance benchmarks on a modern system:

| Domain | Parse Time | Actions | Throughput |
|--------|-----------|---------|------------|
| Simple Robot | ~1-5ms | 4 | >200 parses/sec |
| Blocks World | ~2-8ms | 4 | >125 parses/sec |
| Factory Automation | ~3-12ms | 4 | >80 parses/sec |

## Integration with CI/CD

### GitHub Actions Example
```yaml
- name: Run End-to-End Tests
  run: |
    cargo run --bin comprehensive_tests
    cargo test --test integration_tests
```

### Test Exit Codes
- `0`: All tests passed
- `1`: Some tests failed
- `101`: Panic or critical error

## Troubleshooting

### Common Issues

1. **Missing PDDL Files**
   - Ensure test fixture files exist
   - Check file paths are correct

2. **Parse Failures**
   - Verify PDDL syntax is valid
   - Check for missing requirements declarations

3. **Search Integration Issues**
   - Planning tests may be skipped if search is incomplete
   - This is expected behavior during development

### Log Files
Tests create detailed logs that can help with debugging:
- `test_output.log` - Console output
- `test_results.json` - Structured test results

## Future Enhancements

Planned improvements to the test suite:

1. **Property-based Testing**: Use proptest for generated PDDL domains
2. **Regression Testing**: Compare against known good results
3. **Coverage Analysis**: Ensure comprehensive domain feature coverage
4. **Load Testing**: Test with large-scale domains and problems
5. **Planning Validation**: Full end-to-end planning verification once search is complete
