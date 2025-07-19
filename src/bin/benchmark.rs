use std::fs;
use std::time::Instant;
use temporal_planner::TemporalTask;

fn main() {
    println!("⚡ Temporal Planner Performance Benchmarks");
    println!("{}", "=".repeat(50));
    
    let test_cases = vec![
        ("Small Domain", "tests/fixtures/domains/simple_robot.pddl", "tests/fixtures/problems/simple_delivery.pddl"),
        ("Medium Domain", "tests/fixtures/domains/blocks_world.pddl", "tests/fixtures/problems/stack_blocks.pddl"),
        ("Large Domain", "tests/fixtures/domains/factory_automation.pddl", "tests/fixtures/problems/factory_production.pddl"),
    ];

    for (name, domain_file, problem_file) in test_cases {
        println!("\n📊 Benchmarking: {}", name);
        
        // Read files
        let domain_content = match fs::read_to_string(domain_file) {
            Ok(content) => content,
            Err(e) => {
                println!("   ❌ Failed to read domain file: {}", e);
                continue;
            }
        };
        
        let problem_content = match fs::read_to_string(problem_file) {
            Ok(content) => content,
            Err(e) => {
                println!("   ❌ Failed to read problem file: {}", e);
                continue;
            }
        };

        // Benchmark parsing
        let iterations = 100;
        let mut parse_times = Vec::new();
        
        for _ in 0..iterations {
            let start = Instant::now();
            let _task = TemporalTask::from_pddl(&domain_content, &problem_content);
            parse_times.push(start.elapsed().as_micros());
        }
        
        // Calculate statistics
        let mean_time = parse_times.iter().sum::<u128>() as f64 / parse_times.len() as f64;
        let min_time = *parse_times.iter().min().unwrap() as f64;
        let max_time = *parse_times.iter().max().unwrap() as f64;
        
        // Calculate standard deviation
        let variance = parse_times.iter()
            .map(|&x| {
                let diff = x as f64 - mean_time;
                diff * diff
            })
            .sum::<f64>() / parse_times.len() as f64;
        let std_dev = variance.sqrt();
        
        println!("   Parsing Performance ({} iterations):", iterations);
        println!("     Mean: {:.2}μs ({:.4}ms)", mean_time, mean_time / 1000.0);
        println!("     Min:  {:.2}μs ({:.4}ms)", min_time, min_time / 1000.0);
        println!("     Max:  {:.2}μs ({:.4}ms)", max_time, max_time / 1000.0);
        println!("     Std:  {:.2}μs", std_dev);
        
        // Parse once more to analyze the domain
        let task = TemporalTask::from_pddl(&domain_content, &problem_content);
        let durative_actions = task.actions.iter().filter(|a| a.duration > 1.0).count();
        
        println!("   Domain Analysis:");
        println!("     Actions: {} ({}% durative)", 
                task.actions.len(),
                if task.actions.len() > 0 { 
                    durative_actions * 100 / task.actions.len()
                } else { 
                    0 
                });
        println!("     Initial Facts: {}", task.initial_state.facts.len());
        println!("     Goal Conditions: {}", task.goal_conditions.len());
        
        // Throughput calculation
        let throughput = 1_000_000.0 / mean_time; // parses per second
        println!("     Throughput: {:.1} parses/second", throughput);
    }
    
    // Memory usage benchmark
    println!("\n🔍 Memory Usage Analysis");
    benchmark_memory_usage();
    
    println!("\n{}", "=".repeat(50));
    println!("✅ Benchmarking completed!");
}

fn benchmark_memory_usage() {
    let domain_content = r#"
(define (domain memory-test)
  (:requirements :strips :durative-actions)
  (:predicates (p1) (p2) (p3) (p4) (p5))
  
  (:action action1
    :parameters ()
    :precondition (p1)
    :effect (p2)
  )
  
  (:durative-action long-action
    :parameters ()
    :duration (= ?duration 10.0)
    :condition (at start (p2))
    :effect (at end (p3))
  )
)
"#;

    let problem_content = r#"
(define (problem memory-problem)
  (:domain memory-test)
  (:objects)
  (:init (p1))
  (:goal (p3))
)
"#;

    // Create multiple tasks to test memory usage
    let mut tasks = Vec::new();
    let start_time = Instant::now();
    
    for i in 0..1000 {
        let task = TemporalTask::from_pddl(domain_content, problem_content);
        tasks.push(task);
        
        if i % 100 == 0 {
            let elapsed = start_time.elapsed();
            println!("   Created {} tasks in {:.2}ms", i + 1, elapsed.as_millis());
        }
    }
    
    let total_time = start_time.elapsed();
    println!("   Total: {} tasks created in {:.2}ms", tasks.len(), total_time.as_millis());
    println!("   Average: {:.4}ms per task", total_time.as_millis() as f64 / tasks.len() as f64);
    
    // Keep tasks in memory to prevent optimization
    println!("   Memory footprint: {} tasks in memory", tasks.len());
    drop(tasks); // Explicit cleanup
}
