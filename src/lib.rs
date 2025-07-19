pub mod search;
pub mod heuristics;
pub mod state_space;
pub mod temporal_task;
pub mod scheduler;
pub mod ffi;

pub use temporal_task::{TemporalTask, TemporalAction, Condition, Effect, State};
pub use search::{SearchResult, TemporalAStarSearch, TemporalSearchEngine, Plan};

/// Main API for external applications to interact with the temporal planner
pub struct TemporalPlanner {
    search_engine: Box<dyn TemporalSearchEngine>,
}

impl TemporalPlanner {
    /// Create a new temporal planner instance
    pub fn new() -> Self {
        Self {
            search_engine: Box::new(TemporalAStarSearch::new()),
        }
    }

    /// Parse PDDL domain and problem files from file paths
    pub fn load_pddl_files(&self, domain_path: &str, problem_path: &str) -> Result<TemporalTask, Box<dyn std::error::Error>> {
        let domain_content = std::fs::read_to_string(domain_path)?;
        let problem_content = std::fs::read_to_string(problem_path)?;
        Ok(TemporalTask::from_pddl(&domain_content, &problem_content))
    }

    /// Parse PDDL domain and problem from string content
    pub fn load_pddl_content(&self, domain_content: &str, problem_content: &str) -> TemporalTask {
        TemporalTask::from_pddl(domain_content, problem_content)
    }

    /// Solve a temporal planning task
    pub fn solve(&mut self, task: &TemporalTask) -> SearchResult {
        self.search_engine.search(task)
    }

    /// Complete pipeline: load PDDL files and solve
    pub fn solve_from_files(&mut self, domain_path: &str, problem_path: &str) -> Result<SearchResult, Box<dyn std::error::Error>> {
        let task = self.load_pddl_files(domain_path, problem_path)?;
        Ok(self.solve(&task))
    }

    /// Complete pipeline: load PDDL content and solve
    pub fn solve_from_content(&mut self, domain_content: &str, problem_content: &str) -> SearchResult {
        let task = self.load_pddl_content(domain_content, problem_content);
        self.solve(&task)
    }

    /// Get planner statistics and information
    pub fn get_info(&self) -> PlannerInfo {
        PlannerInfo {
            version: env!("CARGO_PKG_VERSION").to_string(),
            search_algorithm: "Temporal A*".to_string(),
            supports_durative_actions: true,
            supports_numeric_fluents: true,
        }
    }
}

impl Default for TemporalPlanner {
    fn default() -> Self {
        Self::new()
    }
}

/// Information about the planner capabilities
#[derive(Debug, Clone)]
pub struct PlannerInfo {
    pub version: String,
    pub search_algorithm: String,
    pub supports_durative_actions: bool,
    pub supports_numeric_fluents: bool,
}

// Legacy alias for backward compatibility
pub type TemporalFastDownward = TemporalPlanner;