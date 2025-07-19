// f:\common\Source_Code\TemporalFastDownward\temporal_planner\examples\external_integration.rs
//! Example demonstrating how external applications can integrate with the temporal planner

use temporal_planner::{TemporalPlanner, SearchResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Temporal Planner External Integration Example");
    println!("=================================================");

    // 1. Create planner instance
    let mut planner = TemporalPlanner::new();
    
    // 2. Get planner information
    let info = planner.get_info();
    println!("📋 Planner Info:");
    println!("   Version: {}", info.version);
    println!("   Algorithm: {}", info.search_algorithm);
    println!("   Durative Actions: {}", info.supports_durative_actions);
    println!("   Numeric Fluents: {}", info.supports_numeric_fluents);
    println!();

    // 3. Example 1: Load from files
    println!("📁 Example 1: Loading from PDDL files");
    match planner.solve_from_files(
        "tests/fixtures/domains/simple_robot.pddl",
        "tests/fixtures/problems/simple_delivery.pddl"
    ) {
        Ok(SearchResult::Solution(plan)) => {
            println!("   ✅ Solution found with {} actions", plan.actions.len());
            for (i, action) in plan.actions.iter().enumerate() {
                println!("     {}. {} (time: {:.2})", i+1, action.name, action.start_time);
            }
        }
        Ok(SearchResult::Failure) => {
            println!("   ❌ No solution found");
        }
        Err(e) => {
            println!("   ⚠️  Error: {}", e);
        }
    }
    println!();

    // 4. Example 2: Load from content strings
    println!("📝 Example 2: Loading from PDDL content strings");
    let domain_content = r#"
(define (domain simple-example)
  (:requirements :strips :durative-actions)
  (:predicates (at ?x) (goal-reached))
  (:durative-action move
    :parameters ()
    :duration (= ?duration 1.0)
    :condition (at start (at start))
    :effect (and (at end (goal-reached))
                 (at end (not (at start)))))
)
"#;

    let problem_content = r#"
(define (problem simple-problem)
  (:domain simple-example)
  (:init (at start))
  (:goal (goal-reached))
)
"#;

    match planner.solve_from_content(domain_content, problem_content) {
        SearchResult::Solution(plan) => {
            println!("   ✅ Solution found with {} actions", plan.actions.len());
            println!("   Plan cost: {:.2}", plan.cost);
        }
        SearchResult::Failure => {
            println!("   ❌ No solution found");
        }
    }
    println!();

    // 5. Example 3: Parsing only (without solving)
    println!("🔍 Example 3: Parsing PDDL without solving");
    let task = planner.load_pddl_content(domain_content, problem_content);
    println!("   📊 Task Analysis:");
    println!("      Actions: {}", task.actions.len());
    println!("      Goal conditions: {}", task.goal_conditions.len());
    println!("      Initial facts: {}", task.initial_state.facts.len());
    
    for action in &task.actions {
        println!("      Action '{}': duration={:.1}, start_conds={}, end_conds={}", 
                action.name, action.duration, 
                action.conditions_start.len(), 
                action.conditions_end.len());
    }

    println!();
    println!("✅ Integration examples completed successfully!");
    Ok(())
}
