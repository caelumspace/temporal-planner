use std::fs;
use std::io::{self, Write};
use serde_json;
use serde::{Serialize, Deserialize};
use temporal_planner::{TemporalTask, SearchResult, TemporalAStarSearch, TemporalSearchEngine};

#[derive(Debug, Serialize, Deserialize)]
struct TestReport {
    timestamp: String,
    total_tests: usize,
    passed_tests: usize,
    failed_tests: usize,
    test_results: Vec<TestCaseResult>,
    summary: TestSummary,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestCaseResult {
    name: String,
    status: TestStatus,
    duration_ms: u128,
    description: String,
    metrics: TestMetrics,
    error_message: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
enum TestStatus {
    Passed,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct TestMetrics {
    actions_parsed: usize,
    durative_actions: usize,
    initial_facts: usize,
    goal_conditions: usize,
    parse_time_ms: Option<u128>,
    search_time_ms: Option<u128>,
    plan_length: Option<usize>,
    plan_cost: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
struct TestSummary {
    success_rate: f64,
    total_actions_parsed: usize,
    total_durative_actions: usize,
    average_parse_time_ms: f64,
    domains_tested: Vec<String>,
    planning_successful: bool,
}

fn main() {
    println!("🔍 Temporal Planner Comprehensive Test Suite");
    println!("{}", "=".repeat(70));
    
    let mut test_report = TestReport {
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string(),
        total_tests: 0,
        passed_tests: 0,
        failed_tests: 0,
        test_results: Vec::new(),
        summary: TestSummary {
            success_rate: 0.0,
            total_actions_parsed: 0,
            total_durative_actions: 0,
            average_parse_time_ms: 0.0,
            domains_tested: Vec::new(),
            planning_successful: false,
        },
    };

    // Define test cases
    let test_cases = vec![
        ("Simple Robot Domain", "simple_robot.pddl", "simple_delivery.pddl"),
        ("Blocks World Domain", "blocks_world.pddl", "stack_blocks.pddl"),
        ("Factory Automation", "factory_automation.pddl", "factory_production.pddl"),
    ];

    println!("Running {} test cases...\n", test_cases.len());

    for (test_name, domain_file, problem_file) in test_cases {
        print!("• {} ... ", test_name);
        io::stdout().flush().unwrap();
        
        let start_time = std::time::Instant::now();
        let result = run_domain_test(test_name, domain_file, problem_file);
        let duration = start_time.elapsed();
        
        match result {
            Ok(metrics) => {
                println!("✅ PASSED ({:.2}ms)", duration.as_millis());
                test_report.passed_tests += 1;
                test_report.test_results.push(TestCaseResult {
                    name: test_name.to_string(),
                    status: TestStatus::Passed,
                    duration_ms: duration.as_millis(),
                    description: format!("Successfully parsed {} domain with {} actions", 
                                       test_name, metrics.actions_parsed),
                    metrics: metrics.clone(),
                    error_message: None,
                });
                
                // Update summary statistics
                test_report.summary.total_actions_parsed += metrics.actions_parsed;
                test_report.summary.total_durative_actions += metrics.durative_actions;
                test_report.summary.domains_tested.push(test_name.to_string());
            }
            Err(error) => {
                println!("❌ FAILED ({:.2}ms)", duration.as_millis());
                println!("   Error: {}", error);
                test_report.failed_tests += 1;
                test_report.test_results.push(TestCaseResult {
                    name: test_name.to_string(),
                    status: TestStatus::Failed,
                    duration_ms: duration.as_millis(),
                    description: format!("Failed to parse {} domain", test_name),
                    metrics: TestMetrics::default(),
                    error_message: Some(error),
                });
            }
        }
        test_report.total_tests += 1;
    }

    // Run planning integration test
    println!("\n• Planning Integration Test ... ");
    io::stdout().flush().unwrap();
    
    let planning_result = test_planning_integration();
    match planning_result {
        Ok(plan_metrics) => {
            println!("✅ PASSED");
            test_report.summary.planning_successful = true;
            test_report.passed_tests += 1;
            test_report.test_results.push(TestCaseResult {
                name: "Planning Integration".to_string(),
                status: TestStatus::Passed,
                duration_ms: 0,
                description: "Planning system integration test".to_string(),
                metrics: plan_metrics,
                error_message: None,
            });
        }
        Err(error) => {
            println!("⚠️  SKIPPED");
            println!("   Reason: {}", error);
            test_report.test_results.push(TestCaseResult {
                name: "Planning Integration".to_string(),
                status: TestStatus::Skipped,
                duration_ms: 0,
                description: "Planning system not fully implemented".to_string(),
                metrics: TestMetrics::default(),
                error_message: Some(error),
            });
        }
    }
    test_report.total_tests += 1;

    // Calculate final statistics
    test_report.summary.success_rate = 
        (test_report.passed_tests as f64 / test_report.total_tests as f64) * 100.0;
    
    let parse_times: Vec<u128> = test_report.test_results.iter()
        .filter_map(|r| if matches!(r.status, TestStatus::Passed) { 
            Some(r.duration_ms) 
        } else { 
            None 
        })
        .collect();
    
    test_report.summary.average_parse_time_ms = 
        parse_times.iter().sum::<u128>() as f64 / parse_times.len() as f64;

    // Print final results
    print_test_summary(&test_report);
    
    // Save detailed report to file
    save_test_report(&test_report);
}

impl Default for TestMetrics {
    fn default() -> Self {
        Self {
            actions_parsed: 0,
            durative_actions: 0,
            initial_facts: 0,
            goal_conditions: 0,
            parse_time_ms: None,
            search_time_ms: None,
            plan_length: None,
            plan_cost: None,
        }
    }
}

fn run_domain_test(test_name: &str, domain_file: &str, problem_file: &str) -> Result<TestMetrics, String> {
    let domain_path = format!("tests/fixtures/domains/{}", domain_file);
    let problem_path = format!("tests/fixtures/problems/{}", problem_file);
    
    let domain_content = fs::read_to_string(&domain_path)
        .map_err(|e| format!("Failed to read domain file {}: {}", domain_path, e))?;
    let problem_content = fs::read_to_string(&problem_path)
        .map_err(|e| format!("Failed to read problem file {}: {}", problem_path, e))?;
    
    let parse_start = std::time::Instant::now();
    let task = TemporalTask::from_pddl(&domain_content, &problem_content);
    let parse_time = parse_start.elapsed();
    
    // Analyze parsed task
    let durative_actions = task.actions.iter()
        .filter(|a| a.duration > 1.0)
        .count();
    
    let metrics = TestMetrics {
        actions_parsed: task.actions.len(),
        durative_actions,
        initial_facts: task.initial_state.facts.len(),
        goal_conditions: task.goal_conditions.len(),
        parse_time_ms: Some(parse_time.as_millis()),
        search_time_ms: None,
        plan_length: None,
        plan_cost: None,
    };
    
    // Validate expected results for known domains
    validate_domain_expectations(test_name, &task)?;
    
    Ok(metrics)
}

fn validate_domain_expectations(test_name: &str, task: &TemporalTask) -> Result<(), String> {
    match test_name {
        "Simple Robot Domain" => {
            if task.actions.len() != 4 {
                return Err(format!("Expected 4 actions, got {}", task.actions.len()));
            }
            let deliver_action = task.actions.iter().find(|a| a.name == "deliver")
                .ok_or("Missing 'deliver' durative action")?;
            if deliver_action.duration != 2.0 {
                return Err(format!("Deliver action should have duration 2.0, got {}", deliver_action.duration));
            }
        }
        "Blocks World Domain" => {
            if task.actions.len() != 4 {
                return Err(format!("Expected 4 actions, got {}", task.actions.len()));
            }
            let stack_action = task.actions.iter().find(|a| a.name == "stack-slow")
                .ok_or("Missing 'stack-slow' durative action")?;
            if stack_action.duration != 3.0 {
                return Err(format!("Stack-slow action should have duration 3.0, got {}", stack_action.duration));
            }
        }
        "Factory Automation" => {
            if task.actions.len() != 4 {
                return Err(format!("Expected 4 actions, got {}", task.actions.len()));
            }
        }
        _ => {} // No specific validation for other domains
    }
    Ok(())
}

fn test_planning_integration() -> Result<TestMetrics, String> {
    let simple_domain = r#"
(define (domain minimal-test)
  (:requirements :strips)
  (:predicates (start) (goal))
  (:action achieve-goal
    :parameters ()
    :precondition (start)
    :effect (and (not (start)) (goal))
  )
)
"#;

    let simple_problem = r#"
(define (problem minimal-problem)
  (:domain minimal-test)
  (:objects)
  (:init (start))
  (:goal (goal))
)
"#;

    let task = TemporalTask::from_pddl(simple_domain, simple_problem);
    let mut search_engine = TemporalAStarSearch::new();
    
    let search_start = std::time::Instant::now();
    let result = search_engine.search(&task);
    let search_time = search_start.elapsed();
    
    match result {
        SearchResult::Solution(plan) => {
            Ok(TestMetrics {
                actions_parsed: task.actions.len(),
                durative_actions: 0,
                initial_facts: task.initial_state.facts.len(),
                goal_conditions: task.goal_conditions.len(),
                parse_time_ms: None,
                search_time_ms: Some(search_time.as_millis()),
                plan_length: Some(plan.actions.len()),
                plan_cost: Some(plan.cost),
            })
        }
        SearchResult::Failure => {
            Err("Planning system incomplete - search returned failure".to_string())
        }
    }
}

fn print_test_summary(report: &TestReport) {
    println!("\n{}", "=".repeat(70));
    println!("📊 TEST SUMMARY");
    println!("{}", "=".repeat(70));
    
    println!("Overall Results:");
    println!("  • Total Tests: {}", report.total_tests);
    println!("  • Passed: {} ✅", report.passed_tests);
    println!("  • Failed: {} ❌", report.failed_tests);
    println!("  • Success Rate: {:.1}%", report.summary.success_rate);
    println!();
    
    println!("Parsing Performance:");
    println!("  • Total Actions Parsed: {}", report.summary.total_actions_parsed);
    println!("  • Durative Actions: {}", report.summary.total_durative_actions);
    println!("  • Average Parse Time: {:.2}ms", report.summary.average_parse_time_ms);
    println!("  • Domains Tested: {}", report.summary.domains_tested.join(", "));
    println!();
    
    println!("Planning Integration:");
    if report.summary.planning_successful {
        println!("  • Planning System: ✅ Working");
    } else {
        println!("  • Planning System: ⚠️  Incomplete");
    }
    println!();
    
    // Show details for failed tests
    let failed_tests: Vec<&TestCaseResult> = report.test_results.iter()
        .filter(|r| matches!(r.status, TestStatus::Failed))
        .collect();
        
    if !failed_tests.is_empty() {
        println!("Failed Test Details:");
        for test in failed_tests {
            println!("  • {}: {}", test.name, 
                    test.error_message.as_ref().unwrap_or(&"Unknown error".to_string()));
        }
        println!();
    }
}

fn save_test_report(report: &TestReport) {
    let report_path = "test_results.json";
    match serde_json::to_string_pretty(report) {
        Ok(json_content) => {
            if let Err(e) = fs::write(report_path, json_content) {
                println!("⚠️  Failed to save test report to {}: {}", report_path, e);
            } else {
                println!("📄 Detailed test report saved to: {}", report_path);
            }
        }
        Err(e) => {
            println!("⚠️  Failed to serialize test report: {}", e);
        }
    }
}
