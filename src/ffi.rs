// f:\common\Source_Code\TemporalFastDownward\temporal_planner\src\ffi.rs
//! Foreign Function Interface (FFI) for external application integration

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

use crate::{TemporalPlanner, SearchResult};

/// Opaque handle for the temporal planner
pub struct PlannerHandle {
    planner: TemporalPlanner,
}

/// C-compatible result codes
#[repr(C)]
pub enum PlannerResult {
    Success = 0,
    SolutionFound = 1,
    NoSolutionFound = 2,
    ParseError = 3,
    FileError = 4,
    InvalidHandle = 5,
}

/// Create a new temporal planner instance
/// Returns: Opaque handle to the planner, or null on failure
#[no_mangle]
pub extern "C" fn temporal_planner_create() -> *mut PlannerHandle {
    let planner = TemporalPlanner::new();
    Box::into_raw(Box::new(PlannerHandle { planner }))
}

/// Destroy a temporal planner instance
/// Parameters: handle - Planner handle to destroy
#[no_mangle]
pub extern "C" fn temporal_planner_destroy(handle: *mut PlannerHandle) {
    if !handle.is_null() {
        unsafe {
            drop(Box::from_raw(handle));
        }
    }
}

/// Solve a planning problem from PDDL files
/// Parameters:
///   handle - Planner handle
///   domain_path - Path to PDDL domain file
///   problem_path - Path to PDDL problem file
///   plan_length - Output parameter for plan length (can be null)
/// Returns: Result code indicating success/failure
#[no_mangle]
pub extern "C" fn temporal_planner_solve_files(
    handle: *mut PlannerHandle,
    domain_path: *const c_char,
    problem_path: *const c_char,
    plan_length: *mut c_int,
) -> PlannerResult {
    if handle.is_null() || domain_path.is_null() || problem_path.is_null() {
        return PlannerResult::InvalidHandle;
    }

    unsafe {
        let planner_handle = &mut *handle;
        
        let domain_path_str = match CStr::from_ptr(domain_path).to_str() {
            Ok(s) => s,
            Err(_) => return PlannerResult::InvalidHandle,
        };
        
        let problem_path_str = match CStr::from_ptr(problem_path).to_str() {
            Ok(s) => s,
            Err(_) => return PlannerResult::InvalidHandle,
        };

        match planner_handle.planner.solve_from_files(domain_path_str, problem_path_str) {
            Ok(SearchResult::Solution(plan)) => {
                if !plan_length.is_null() {
                    *plan_length = plan.actions.len() as c_int;
                }
                PlannerResult::SolutionFound
            }
            Ok(SearchResult::Failure) => PlannerResult::NoSolutionFound,
            Err(_) => PlannerResult::FileError,
        }
    }
}

/// Solve a planning problem from PDDL content strings
/// Parameters:
///   handle - Planner handle
///   domain_content - PDDL domain content as C string
///   problem_content - PDDL problem content as C string
///   plan_length - Output parameter for plan length (can be null)
/// Returns: Result code indicating success/failure
#[no_mangle]
pub extern "C" fn temporal_planner_solve_content(
    handle: *mut PlannerHandle,
    domain_content: *const c_char,
    problem_content: *const c_char,
    plan_length: *mut c_int,
) -> PlannerResult {
    if handle.is_null() || domain_content.is_null() || problem_content.is_null() {
        return PlannerResult::InvalidHandle;
    }

    unsafe {
        let planner_handle = &mut *handle;
        
        let domain_str = match CStr::from_ptr(domain_content).to_str() {
            Ok(s) => s,
            Err(_) => return PlannerResult::InvalidHandle,
        };
        
        let problem_str = match CStr::from_ptr(problem_content).to_str() {
            Ok(s) => s,
            Err(_) => return PlannerResult::InvalidHandle,
        };

        match planner_handle.planner.solve_from_content(domain_str, problem_str) {
            SearchResult::Solution(plan) => {
                if !plan_length.is_null() {
                    *plan_length = plan.actions.len() as c_int;
                }
                PlannerResult::SolutionFound
            }
            SearchResult::Failure => PlannerResult::NoSolutionFound,
        }
    }
}

/// Get planner version information
/// Returns: C string with version info (caller must free)
#[no_mangle]
pub extern "C" fn temporal_planner_get_version() -> *mut c_char {
    let version = env!("CARGO_PKG_VERSION");
    match CString::new(version) {
        Ok(c_string) => c_string.into_raw(),
        Err(_) => ptr::null_mut(),
    }
}

/// Free a C string returned by the planner
/// Parameters: str_ptr - Pointer to C string to free
#[no_mangle]
pub extern "C" fn temporal_planner_free_string(str_ptr: *mut c_char) {
    if !str_ptr.is_null() {
        unsafe {
            drop(CString::from_raw(str_ptr));
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_ffi_lifecycle() {
        // Test creating and destroying planner
        let handle = temporal_planner_create();
        assert!(!handle.is_null());
        
        temporal_planner_destroy(handle);
    }

    #[test]
    fn test_ffi_version() {
        let version_ptr = temporal_planner_get_version();
        assert!(!version_ptr.is_null());
        
        unsafe {
            let version_str = CStr::from_ptr(version_ptr).to_str().unwrap();
            assert!(!version_str.is_empty());
        }
        
        temporal_planner_free_string(version_ptr);
    }
}
