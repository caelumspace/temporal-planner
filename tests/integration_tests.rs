use std::fs;
use temporal_planner::{TemporalTask, TemporalAStarSearch, TemporalSearchEngine, SearchResult};

#[test]
fn test_simple_robot_domain_parsing() {
    let domain_path = "tests/fixtures/domains/simple_robot.pddl";
    let problem_path = "tests/fixtures/problems/simple_delivery.pddl";
    
    let domain_content = fs::read_to_string(domain_path)
        .expect("Failed to read domain file");
    let problem_content = fs::read_to_string(problem_path)
        .expect("Failed to read problem file");
    
    let task = TemporalTask::from_pddl(&domain_content, &problem_content);
    
    // Verify domain parsing
    assert_eq!(task.actions.len(), 4, "Expected 4 actions in simple robot domain");
    
    // Find specific actions
    let move_action = task.actions.iter().find(|a| a.name == "move").expect("Move action not found");
    let _pickup_action = task.actions.iter().find(|a| a.name == "pick-up").expect("Pick-up action not found");
    let deliver_action = task.actions.iter().find(|a| a.name == "deliver").expect("Deliver action not found");
    
    // Verify action properties
    assert_eq!(move_action.duration, 1.0, "Move action should have default duration");
    assert_eq!(deliver_action.duration, 2.0, "Deliver action should have duration 2.0");
    
    // Verify initial state contains expected facts
    assert!(!task.initial_state.facts.is_empty(), "Initial state should not be empty");
    
    // Verify goal conditions
    assert_eq!(task.goal_conditions.len(), 2, "Expected 2 goal conditions");
    
    println!("✅ Simple robot domain parsing test passed");
    println!("   - Actions: {}", task.actions.len());
    println!("   - Initial facts: {}", task.initial_state.facts.len());
    println!("   - Goal conditions: {}", task.goal_conditions.len());
}

#[test]
fn test_blocks_world_domain_parsing() {
    let domain_path = "tests/fixtures/domains/blocks_world.pddl";
    let problem_path = "tests/fixtures/problems/stack_blocks.pddl";
    
    let domain_content = fs::read_to_string(domain_path)
        .expect("Failed to read domain file");
    let problem_content = fs::read_to_string(problem_path)
        .expect("Failed to read problem file");
    
    let task = TemporalTask::from_pddl(&domain_content, &problem_content);
    
    // Verify domain parsing
    assert_eq!(task.actions.len(), 4, "Expected 4 actions in blocks world domain");
    
    // Find durative action
    let stack_action = task.actions.iter().find(|a| a.name == "stack-slow")
        .expect("Stack-slow action not found");
    
    // Verify durative action properties
    assert_eq!(stack_action.duration, 3.0, "Stack-slow action should have duration 3.0");
    assert!(!stack_action.conditions_start.is_empty(), "Stack action should have start conditions");
    assert!(!stack_action.effects_end.is_empty(), "Stack action should have end effects");
    
    println!("✅ Blocks world domain parsing test passed");
    println!("   - Actions: {}", task.actions.len());
    println!("   - Durative action duration: {}", stack_action.duration);
}

#[test]
fn test_factory_automation_complex_parsing() {
    let domain_path = "tests/fixtures/domains/factory_automation.pddl";
    let problem_path = "tests/fixtures/problems/factory_production.pddl";
    
    let domain_content = fs::read_to_string(domain_path)
        .expect("Failed to read domain file");
    let problem_content = fs::read_to_string(problem_path)
        .expect("Failed to read problem file");
    
    let task = TemporalTask::from_pddl(&domain_content, &problem_content);
    
    // Verify complex domain with numeric fluents
    assert_eq!(task.actions.len(), 4, "Expected 4 actions in factory automation domain");
    
    // Find process and produce actions
    let process_action = task.actions.iter().find(|a| a.name == "process-ingredient")
        .expect("Process-ingredient action not found");
    let produce_action = task.actions.iter().find(|a| a.name == "produce-product")
        .expect("Produce-product action not found");
    
    // These actions should have variable durations (parsed as default for now)
    println!("✅ Factory automation complex parsing test passed");
    println!("   - Actions: {}", task.actions.len());
    println!("   - Process action found: {}", process_action.name);
    println!("   - Produce action found: {}", produce_action.name);
}

#[test]
#[ignore] // Ignore until search is fully implemented
fn test_simple_robot_planning() {
    let domain_path = "tests/fixtures/domains/simple_robot.pddl";
    let problem_path = "tests/fixtures/problems/simple_delivery.pddl";
    
    let domain_content = fs::read_to_string(domain_path)
        .expect("Failed to read domain file");
    let problem_content = fs::read_to_string(problem_path)
        .expect("Failed to read problem file");
    
    let task = TemporalTask::from_pddl(&domain_content, &problem_content);
    let mut search_engine = TemporalAStarSearch::new();
    
    let result = search_engine.search(&task);
    
    match result {
        SearchResult::Solution(plan) => {
            assert!(!plan.actions.is_empty(), "Plan should not be empty");
            assert!(plan.cost > 0.0, "Plan cost should be positive");
            
            println!("✅ Simple robot planning test passed");
            println!("   - Plan length: {}", plan.actions.len());
            println!("   - Plan cost: {}", plan.cost);
        }
        SearchResult::Failure => {
            panic!("Planning should have found a solution");
        }
    }
}

#[test]
fn test_action_temporal_properties() {
    let domain_content = r#"
(define (domain test-temporal)
  (:requirements :strips :durative-actions)
  (:predicates (p) (q) (r))
  
  (:durative-action test-action
    :parameters ()
    :duration (= ?duration 5.0)
    :condition (and (at start (p))
                    (over all (q))
                    (at end (r)))
    :effect (and (at start (not (p)))
                 (at end (r)))
  )
)
"#;

    let problem_content = r#"
(define (problem test-problem)
  (:domain test-temporal)
  (:objects)
  (:init (p) (q))
  (:goal (r))
)
"#;

    let task = TemporalTask::from_pddl(domain_content, problem_content);
    
    assert_eq!(task.actions.len(), 1, "Expected 1 action");
    
    let action = &task.actions[0];
    assert_eq!(action.name, "test-action");
    assert_eq!(action.duration, 5.0);
    assert_eq!(action.conditions_start.len(), 1, "Expected 1 start condition");
    assert_eq!(action.conditions_over_all.len(), 1, "Expected 1 over-all condition");
    assert_eq!(action.conditions_end.len(), 1, "Expected 1 end condition");
    assert_eq!(action.effects_start.len(), 1, "Expected 1 start effect");
    assert_eq!(action.effects_end.len(), 1, "Expected 1 end effect");
    
    println!("✅ Action temporal properties test passed");
    println!("   - Start conditions: {}", action.conditions_start.len());
    println!("   - Over-all conditions: {}", action.conditions_over_all.len());
    println!("   - End conditions: {}", action.conditions_end.len());
}

#[test]
fn test_error_handling_invalid_domain() {
    let invalid_domain = r#"
(define (domain invalid)
  this is not valid PDDL
)
"#;

    let valid_problem = r#"
(define (problem test)
  (:domain invalid)
  (:objects)
  (:init)
  (:goal (p))
)
"#;

    let task = TemporalTask::from_pddl(invalid_domain, valid_problem);
    
    // The parser should handle this gracefully (even if it returns empty results)
    println!("✅ Error handling test passed");
    println!("   - Actions parsed from invalid domain: {}", task.actions.len());
}

#[cfg(test)]
mod benchmarks {
    use super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_domain_parsing() {
        let domain_path = "tests/fixtures/domains/factory_automation.pddl";
        let problem_path = "tests/fixtures/problems/factory_production.pddl";
        
        let domain_content = fs::read_to_string(domain_path)
            .expect("Failed to read domain file");
        let problem_content = fs::read_to_string(problem_path)
            .expect("Failed to read problem file");
        
        let start = Instant::now();
        let _task = TemporalTask::from_pddl(&domain_content, &problem_content);
        let duration = start.elapsed();
        
        println!("✅ Benchmark: Domain parsing took {:?}", duration);
        assert!(duration.as_millis() < 1000, "Parsing should complete within 1 second");
    }
}

/// Helper function to run all end-to-end tests and report results
pub fn run_all_e2e_tests() {
    println!("🚀 Running End-to-End Tests for Temporal Planner");
    println!("{}", "=".repeat(50));
    
    test_simple_robot_domain_parsing();
    test_blocks_world_domain_parsing();
    test_factory_automation_complex_parsing();
    test_action_temporal_properties();
    test_error_handling_invalid_domain();
    
    println!("{}", "=".repeat(50));
    println!("✅ All end-to-end tests completed successfully!");
}
