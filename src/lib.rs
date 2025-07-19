pub mod search;
pub mod heuristics;
pub mod state_space;
pub mod temporal_task;
pub mod scheduler;

pub use temporal_task::TemporalTask;
pub use search::{SearchResult, TemporalAStarSearch, TemporalSearchEngine, Plan};

pub struct TemporalFastDownward {
    search_engine: Box<dyn TemporalSearchEngine>,
}

impl TemporalFastDownward {
    pub fn new() -> Self {
        Self {
            search_engine: Box::new(TemporalAStarSearch::new()),
        }
    }

    pub fn solve(&mut self) -> SearchResult {
        let task = temporal_task::TemporalTask::new();
        self.search_engine.search(&task)
    }
}