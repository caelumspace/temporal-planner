use temporal_planner::TemporalTask;

fn main() {
    let domain = r#"
(define (domain robotic-control)
  (:requirements :strips :typing :durative-actions)

  (:types
    entity device position orientation object
  )

  (:predicates
    (at-position ?e - entity ?p - position)
    (has-orientation ?e - entity ?o - orientation)
    (device-available ?d - device)
    (area-scanned ?e - entity)
    (device-deployed ?e - entity)
    (object-manipulated ?e - entity)
  )

  (:action move-to-position
    :parameters (?e - entity ?from - position ?to - position)
    :precondition (and (at-position ?e ?from) (device-available ?e))
    :effect (and (not (at-position ?e ?from))
                 (at-position ?e ?to))
  )

  (:durative-action scan-area-durative
    :parameters (?e - entity ?area - position)
    :duration (= ?duration 5.0)
    :condition (and (at start (at-position ?e ?area))
                    (over all (device-available ?e)))
    :effect (at end (area-scanned ?e))
  )

  (:durative-action deploy-device-durative
    :parameters (?e - entity ?d - device ?pos - position)
    :duration (= ?duration 3.0)
    :condition (and (at start (at-position ?e ?pos))
                    (at start (device-available ?d)))
    :effect (and (at end (device-deployed ?e))
                 (at start (not (device-available ?d))))
  )
)
"#;

    let problem = r#"
(define (problem simple-robotic-problem)
  (:domain robotic-control)
  (:objects
    robot1 - entity
    device1 - device
    pos1 pos2 - position
    north south - orientation
    obj1 - object
  )
  (:init
    (at-position robot1 pos1)
    (has-orientation robot1 north)
    (device-available device1)
  )
  (:goal
    (and (at-position robot1 pos2)
         (area-scanned robot1))
  )
)
"#;

    println!("Parsing PDDL domain and problem with durative actions...");
    
    let task = TemporalTask::from_pddl(domain, problem);
    
    println!("Parsed {} actions:", task.actions.len());
    for action in &task.actions {
        println!("  - Action: {}", action.name);
        println!("    Duration: {}", action.duration);
        println!("    Start conditions: {}", action.conditions_start.len());
        println!("    Over-all conditions: {}", action.conditions_over_all.len());
        println!("    End conditions: {}", action.conditions_end.len());
        println!("    Start effects: {}", action.effects_start.len());
        println!("    End effects: {}", action.effects_end.len());
        
        // Show detailed conditions and effects
        if !action.conditions_start.is_empty() {
            println!("      Start conditions:");
            for cond in &action.conditions_start {
                println!("        - {}: {:?} (negative: {})", cond.predicate, cond.args, cond.is_negative);
            }
        }
        if !action.conditions_over_all.is_empty() {
            println!("      Over-all conditions:");
            for cond in &action.conditions_over_all {
                println!("        - {}: {:?} (negative: {})", cond.predicate, cond.args, cond.is_negative);
            }
        }
        if !action.effects_start.is_empty() {
            println!("      Start effects:");
            for eff in &action.effects_start {
                println!("        - {}: {:?} (delete: {})", eff.predicate, eff.args, eff.is_delete);
            }
        }
        if !action.effects_end.is_empty() {
            println!("      End effects:");
            for eff in &action.effects_end {
                println!("        - {}: {:?} (delete: {})", eff.predicate, eff.args, eff.is_delete);
            }
        }
        println!();
    }
    
    println!("Initial state facts: {}", task.initial_state.facts.len());
    println!("Goal conditions: {}", task.goal_conditions.len());
    for goal in &task.goal_conditions {
        println!("  - Goal: {}: {:?} (negative: {})", goal.predicate, goal.args, goal.is_negative);
    }
}
