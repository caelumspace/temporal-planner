// f:\common\Source_Code\TemporalFastDownward\temporal_planner\examples\simple_library_usage.rs
//! Minimal example showing how to use temporal_planner as a library dependency

use temporal_planner::{TemporalPlanner, SearchResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🦀 Using Temporal Planner as a Rust Library");
    println!("=============================================");

    // 1. Create planner instance - that's it!
    let mut planner = TemporalPlanner::new();

    // 2. Get some basic info
    let info = planner.get_info();
    println!("📋 Using {} v{}", info.search_algorithm, info.version);
    println!("   Supports durative actions: {}", info.supports_durative_actions);
    println!();

    // 3. Define simple PDDL content inline
    let domain = r#"
(define (domain simple-robot)
  (:requirements :strips :durative-actions)
  (:predicates (robot-at ?loc) (package-at ?loc) (delivered))
  
  (:durative-action deliver-package
    :parameters ()
    :duration (= ?duration 2.0)
    :condition (and (at start (robot-at depot))
                    (at start (package-at depot)))
    :effect (and (at end (delivered))
                 (at end (not (package-at depot)))))
)
"#;

    let problem = r#"
(define (problem delivery-task)
  (:domain simple-robot)
  (:init (robot-at depot) (package-at depot))
  (:goal (delivered))
)
"#;

    // 4. One-liner to solve the planning problem
    match planner.solve_from_content(domain, problem) {
        SearchResult::Solution(plan) => {
            println!("✅ Success! Found plan with {} actions:", plan.actions.len());
            println!("   Plan cost: {:.2}", plan.cost);
            
            for (i, &action_idx) in plan.actions.iter().enumerate() {
                println!("   {}. Action index {} (cost contributed: {:.2})", 
                    i + 1, action_idx, plan.cost / plan.actions.len() as f64);
            }
        }
        SearchResult::Failure => {
            println!("❌ No solution found");
        }
    }

    println!();
    println!("🎉 That's how easy it is to use as a library!");
    
    Ok(())
}
