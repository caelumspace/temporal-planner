#!/usr/bin/env python3
"""
Python wrapper for the Temporal Planner FFI
Demonstrates how to call the temporal planner from Python
"""

import ctypes
import os
from ctypes import c_char_p, c_int, c_void_p, POINTER
from enum import IntEnum

class PlannerResult(IntEnum):
    SUCCESS = 0
    SOLUTION_FOUND = 1
    NO_SOLUTION_FOUND = 2
    PARSE_ERROR = 3
    FILE_ERROR = 4
    INVALID_HANDLE = 5

class TemporalPlannerWrapper:
    """Python wrapper for the Rust temporal planner"""
    
    def __init__(self, lib_path=None):
        """Initialize the wrapper with the dynamic library"""
        if lib_path is None:
            # Try to find the library in common locations
            possible_paths = [
                "./target/release/libtemporal_planner.so",  # Linux
                "./target/release/libtemporal_planner.dylib",  # macOS
                "./target/release/temporal_planner.dll",  # Windows
                "./target/debug/libtemporal_planner.so",  # Linux debug
                "./target/debug/libtemporal_planner.dylib",  # macOS debug
                "./target/debug/temporal_planner.dll",  # Windows debug
            ]
            
            for path in possible_paths:
                if os.path.exists(path):
                    lib_path = path
                    break
            
            if lib_path is None:
                raise FileNotFoundError("Could not find temporal planner library")
        
        self.lib = ctypes.CDLL(lib_path)
        self._setup_function_signatures()
        self.handle = None
    
    def _setup_function_signatures(self):
        """Set up the function signatures for the FFI calls"""
        # temporal_planner_create
        self.lib.temporal_planner_create.argtypes = []
        self.lib.temporal_planner_create.restype = c_void_p
        
        # temporal_planner_destroy
        self.lib.temporal_planner_destroy.argtypes = [c_void_p]
        self.lib.temporal_planner_destroy.restype = None
        
        # temporal_planner_solve_files
        self.lib.temporal_planner_solve_files.argtypes = [
            c_void_p, c_char_p, c_char_p, POINTER(c_int)
        ]
        self.lib.temporal_planner_solve_files.restype = c_int
        
        # temporal_planner_solve_content
        self.lib.temporal_planner_solve_content.argtypes = [
            c_void_p, c_char_p, c_char_p, POINTER(c_int)
        ]
        self.lib.temporal_planner_solve_content.restype = c_int
        
        # temporal_planner_get_version
        self.lib.temporal_planner_get_version.argtypes = []
        self.lib.temporal_planner_get_version.restype = c_char_p
        
        # temporal_planner_free_string
        self.lib.temporal_planner_free_string.argtypes = [c_char_p]
        self.lib.temporal_planner_free_string.restype = None
    
    def __enter__(self):
        """Context manager entry"""
        self.handle = self.lib.temporal_planner_create()
        if not self.handle:
            raise RuntimeError("Failed to create temporal planner instance")
        return self
    
    def __exit__(self, exc_type, exc_val, exc_tb):
        """Context manager exit"""
        if self.handle:
            self.lib.temporal_planner_destroy(self.handle)
            self.handle = None
    
    def get_version(self):
        """Get the planner version"""
        version_ptr = self.lib.temporal_planner_get_version()
        if version_ptr:
            version = ctypes.string_at(version_ptr).decode('utf-8')
            self.lib.temporal_planner_free_string(version_ptr)
            return version
        return "Unknown"
    
    def solve_files(self, domain_path, problem_path):
        """
        Solve a planning problem from PDDL files
        
        Args:
            domain_path: Path to the PDDL domain file
            problem_path: Path to the PDDL problem file
            
        Returns:
            tuple: (result_code, plan_length)
        """
        if not self.handle:
            raise RuntimeError("Planner not initialized")
        
        plan_length = c_int(0)
        result = self.lib.temporal_planner_solve_files(
            self.handle,
            domain_path.encode('utf-8'),
            problem_path.encode('utf-8'),
            ctypes.byref(plan_length)
        )
        
        return PlannerResult(result), plan_length.value
    
    def solve_content(self, domain_content, problem_content):
        """
        Solve a planning problem from PDDL content strings
        
        Args:
            domain_content: PDDL domain content as string
            problem_content: PDDL problem content as string
            
        Returns:
            tuple: (result_code, plan_length)
        """
        if not self.handle:
            raise RuntimeError("Planner not initialized")
        
        plan_length = c_int(0)
        result = self.lib.temporal_planner_solve_content(
            self.handle,
            domain_content.encode('utf-8'),
            problem_content.encode('utf-8'),
            ctypes.byref(plan_length)
        )
        
        return PlannerResult(result), plan_length.value

def main():
    """Example usage of the Python wrapper"""
    print("🐍 Python Temporal Planner Integration Example")
    print("==============================================")
    
    try:
        with TemporalPlannerWrapper() as planner:
            # Get version info
            version = planner.get_version()
            print(f"📋 Planner Version: {version}")
            print()
            
            # Example 1: Solve from files
            print("📁 Example 1: Solving from PDDL files")
            try:
                result, plan_length = planner.solve_files(
                    "tests/fixtures/domains/simple_robot.pddl",
                    "tests/fixtures/problems/simple_delivery.pddl"
                )
                
                if result == PlannerResult.SOLUTION_FOUND:
                    print(f"   ✅ Solution found with {plan_length} actions")
                elif result == PlannerResult.NO_SOLUTION_FOUND:
                    print("   ❌ No solution found")
                else:
                    print(f"   ⚠️  Error: {result}")
            except Exception as e:
                print(f"   ⚠️  File error: {e}")
            
            print()
            
            # Example 2: Solve from content
            print("📝 Example 2: Solving from PDDL content")
            domain_content = """
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
"""
            
            problem_content = """
(define (problem simple-problem)
  (:domain simple-example)
  (:init (at start))
  (:goal (goal-reached))
)
"""
            
            result, plan_length = planner.solve_content(domain_content, problem_content)
            
            if result == PlannerResult.SOLUTION_FOUND:
                print(f"   ✅ Solution found with {plan_length} actions")
            elif result == PlannerResult.NO_SOLUTION_FOUND:
                print("   ❌ No solution found")
            else:
                print(f"   ⚠️  Error: {result}")
            
            print()
            print("✅ Python integration example completed!")
            
    except Exception as e:
        print(f"❌ Error: {e}")

if __name__ == "__main__":
    main()
