/**
 * @file cpp_integration_example.cpp
 * @brief C++ integration example for the temporal planner
 */

#include <iostream>
#include <string>
#include <stdexcept>
#include "../include/temporal_planner.h"

// Function to convert result code to string
std::string resultToString(PlannerResult result) {
    switch (result) {
        case PLANNER_SUCCESS: return "Success";
        case PLANNER_SOLUTION_FOUND: return "Solution Found";
        case PLANNER_NO_SOLUTION: return "No Solution";
        case PLANNER_PARSE_ERROR: return "Parse Error";
        case PLANNER_FILE_ERROR: return "File Error";
        case PLANNER_INVALID_HANDLE: return "Invalid Handle";
        default: return "Unknown";
    }
}

int main() {
    std::cout << "🔧 C++ Temporal Planner Integration Example\n";
    std::cout << "==========================================\n\n";

    try {
        // Create planner instance using C++ wrapper
        TemporalPlannerCpp planner;

        // Get version information
        std::string version = planner.getVersion();
        std::cout << "📋 Planner Version: " << version << "\n\n";

        // Example 1: Solve from files
        std::cout << "📁 Example 1: Solving from PDDL files\n";
        try {
            auto [result, planLength] = planner.solveFiles(
                "tests/fixtures/domains/simple_robot.pddl",
                "tests/fixtures/problems/simple_delivery.pddl"
            );

            std::cout << "   Result: " << resultToString(result) << "\n";
            if (result == PLANNER_SOLUTION_FOUND) {
                std::cout << "   ✅ Plan length: " << planLength << " actions\n";
            }
        } catch (const std::exception& e) {
            std::cout << "   ⚠️  Error: " << e.what() << "\n";
        }

        std::cout << "\n";

        // Example 2: Solve from content
        std::cout << "📝 Example 2: Solving from PDDL content\n";
        
        std::string domainContent = R"(
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
)";

        std::string problemContent = R"(
(define (problem simple-problem)
  (:domain simple-example)
  (:init (at start))
  (:goal (goal-reached))
)
)";

        auto [result2, planLength2] = planner.solveContent(domainContent, problemContent);
        
        std::cout << "   Result: " << resultToString(result2) << "\n";
        if (result2 == PLANNER_SOLUTION_FOUND) {
            std::cout << "   ✅ Plan length: " << planLength2 << " actions\n";
        }

        std::cout << "\n✅ C++ integration example completed!\n";

    } catch (const std::exception& e) {
        std::cerr << "❌ Error: " << e.what() << "\n";
        return 1;
    }

    return 0;
}

/**
 * Compilation instructions:
 * 
 * 1. First, build the Rust library with C-compatible exports:
 *    cargo build --release --features capi
 * 
 * 2. Then compile this C++ example:
 *    g++ -std=c++17 -o cpp_example cpp_integration_example.cpp -L./target/release -ltemporal_planner
 * 
 * 3. Run the example:
 *    ./cpp_example
 * 
 * Note: You may need to adjust library paths and names based on your system.
 */
