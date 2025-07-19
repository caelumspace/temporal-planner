/**
 * @file temporal_planner.h
 * @brief C/C++ Header for Temporal Planner FFI Integration
 * 
 * This header provides C-compatible function declarations for integrating
 * the Rust temporal planner with C/C++ applications.
 */

#ifndef TEMPORAL_PLANNER_H
#define TEMPORAL_PLANNER_H

#ifdef __cplusplus
extern "C" {
#endif

/**
 * @brief Opaque handle for the temporal planner instance
 */
typedef struct PlannerHandle PlannerHandle;

/**
 * @brief Result codes returned by planner functions
 */
typedef enum {
    PLANNER_SUCCESS = 0,           /**< Operation completed successfully */
    PLANNER_SOLUTION_FOUND = 1,    /**< Planning solution found */
    PLANNER_NO_SOLUTION = 2,       /**< No solution exists for the problem */
    PLANNER_PARSE_ERROR = 3,       /**< Error parsing PDDL content */
    PLANNER_FILE_ERROR = 4,        /**< Error reading PDDL files */
    PLANNER_INVALID_HANDLE = 5     /**< Invalid planner handle */
} PlannerResult;

/**
 * @brief Create a new temporal planner instance
 * @return Opaque handle to the planner, or NULL on failure
 */
PlannerHandle* temporal_planner_create(void);

/**
 * @brief Destroy a temporal planner instance
 * @param handle Planner handle to destroy
 */
void temporal_planner_destroy(PlannerHandle* handle);

/**
 * @brief Solve a planning problem from PDDL files
 * @param handle Planner handle
 * @param domain_path Path to PDDL domain file
 * @param problem_path Path to PDDL problem file
 * @param plan_length Output parameter for plan length (can be NULL)
 * @return Result code indicating success/failure
 */
PlannerResult temporal_planner_solve_files(
    PlannerHandle* handle,
    const char* domain_path,
    const char* problem_path,
    int* plan_length
);

/**
 * @brief Solve a planning problem from PDDL content strings
 * @param handle Planner handle
 * @param domain_content PDDL domain content as C string
 * @param problem_content PDDL problem content as C string
 * @param plan_length Output parameter for plan length (can be NULL)
 * @return Result code indicating success/failure
 */
PlannerResult temporal_planner_solve_content(
    PlannerHandle* handle,
    const char* domain_content,
    const char* problem_content,
    int* plan_length
);

/**
 * @brief Get planner version information
 * @return C string with version info (caller must free with temporal_planner_free_string)
 */
char* temporal_planner_get_version(void);

/**
 * @brief Free a C string returned by the planner
 * @param str_ptr Pointer to C string to free
 */
void temporal_planner_free_string(char* str_ptr);

#ifdef __cplusplus
}

/**
 * @brief C++ wrapper class for the temporal planner
 */
class TemporalPlannerCpp {
private:
    PlannerHandle* handle;

public:
    /**
     * @brief Constructor - creates a new planner instance
     * @throws std::runtime_error if planner creation fails
     */
    TemporalPlannerCpp() : handle(temporal_planner_create()) {
        if (!handle) {
            throw std::runtime_error("Failed to create temporal planner");
        }
    }

    /**
     * @brief Destructor - destroys the planner instance
     */
    ~TemporalPlannerCpp() {
        if (handle) {
            temporal_planner_destroy(handle);
        }
    }

    /**
     * @brief Get planner version
     * @return Version string
     */
    std::string getVersion() {
        char* version_cstr = temporal_planner_get_version();
        if (version_cstr) {
            std::string version(version_cstr);
            temporal_planner_free_string(version_cstr);
            return version;
        }
        return "Unknown";
    }

    /**
     * @brief Solve planning problem from files
     * @param domainPath Path to domain file
     * @param problemPath Path to problem file
     * @return Pair of (result_code, plan_length)
     */
    std::pair<PlannerResult, int> solveFiles(const std::string& domainPath, 
                                           const std::string& problemPath) {
        int planLength = 0;
        PlannerResult result = temporal_planner_solve_files(
            handle, domainPath.c_str(), problemPath.c_str(), &planLength);
        return {result, planLength};
    }

    /**
     * @brief Solve planning problem from content strings
     * @param domainContent PDDL domain content
     * @param problemContent PDDL problem content
     * @return Pair of (result_code, plan_length)
     */
    std::pair<PlannerResult, int> solveContent(const std::string& domainContent,
                                             const std::string& problemContent) {
        int planLength = 0;
        PlannerResult result = temporal_planner_solve_content(
            handle, domainContent.c_str(), problemContent.c_str(), &planLength);
        return {result, planLength};
    }

    // Disable copy constructor and assignment operator
    TemporalPlannerCpp(const TemporalPlannerCpp&) = delete;
    TemporalPlannerCpp& operator=(const TemporalPlannerCpp&) = delete;
};

#endif

#endif /* TEMPORAL_PLANNER_H */
