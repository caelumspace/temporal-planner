// f:\common\Source_Code\TemporalFastDownward\rust\src\temporal_planner\search.rs
use super::state_space::{StateSpace, TemporalState};
use super::temporal_task::TemporalTask;
use std::collections::{BinaryHeap, HashMap};
use std::cmp::Ordering;

#[derive(Debug, Clone)]
pub struct Plan {
    pub actions: Vec<usize>,
    pub cost: f64,
}

#[derive(Debug, Clone)]
pub enum SearchResult {
    Solution(Plan),
    Failure,
}

pub trait TemporalSearchEngine {
    fn search(&mut self, task: &TemporalTask) -> SearchResult;
}

#[derive(Clone)]
struct SearchNode {
    state: TemporalState,
    g_value: f64,
    h_value: f64,
    parent: Option<Box<SearchNode>>,
    action_idx: Option<usize>,
}

impl PartialEq for SearchNode {
    fn eq(&self, other: &Self) -> bool {
        self.f_value() == other.f_value()
    }
}

impl Eq for SearchNode {}

impl Ord for SearchNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.f_value().partial_cmp(&self.f_value()).unwrap()
    }
}

impl PartialOrd for SearchNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl SearchNode {
    fn f_value(&self) -> f64 {
        self.g_value + self.h_value
    }
}

pub struct TemporalAStarSearch {
    heuristic: Box<dyn super::heuristics::TemporalHeuristic>,
}

impl TemporalAStarSearch {
    pub fn new() -> Self {
        Self {
            heuristic: Box::new(super::heuristics::TemporalFFHeuristic::new()),
        }
    }
}

impl TemporalSearchEngine for TemporalAStarSearch {
    fn search(&mut self, task: &TemporalTask) -> SearchResult {
        let state_space = StateSpace::new((*task).clone());
        let initial_state = TemporalState {
            classical_state: task.initial_state.clone(),
            scheduled_effects: Vec::new(),
            time: 0.0,
        };

        let mut open_list = BinaryHeap::new();
        let mut closed_list = HashMap::new();

        let initial_node = SearchNode {
            state: initial_state.clone(),
            g_value: 0.0,
            h_value: self.heuristic.compute(&initial_state, task),
            parent: None,
            action_idx: None,
        };

        open_list.push(initial_node);

        while let Some(node) = open_list.pop() {
            // Check if goal reached
            if self.is_goal(&node.state, task) {
                return self.extract_plan(&node);
            }

            // Skip if already expanded
            if closed_list.contains_key(&node.state.classical_state) {
                continue;
            }

            closed_list.insert(node.state.classical_state.clone(), node.g_value);

            // Process scheduled effects
            let processed_state = self.process_scheduled_effects(&node.state);

            // Generate successors
            for (action_idx, start_time) in state_space.get_applicable_actions(&processed_state) {
                let successor_state = state_space.apply_action(&processed_state, action_idx, start_time);
                
                let g_value = node.g_value + (successor_state.time - node.state.time);
                let h_value = self.heuristic.compute(&successor_state, task);

                let successor_node = SearchNode {
                    state: successor_state,
                    g_value,
                    h_value,
                    parent: Some(Box::new(node.clone())),
                    action_idx: Some(action_idx),
                };

                open_list.push(successor_node);
            }
        }

        SearchResult::Failure
    }
}

impl TemporalAStarSearch {
    fn is_goal(&self, state: &TemporalState, _task: &TemporalTask) -> bool {
        // Check if all goal conditions are satisfied
        // and no scheduled effects remain
        state.scheduled_effects.is_empty() 
            // TODO: Implement proper goal condition checking
            // For now, just check if no scheduled effects remain
    }

    fn process_scheduled_effects(&self, state: &TemporalState) -> TemporalState {
        let mut new_state = state.clone();
        
        // Find next time point
        let next_time = new_state.scheduled_effects
            .iter()
            .map(|e| e.time)
            .min_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap_or(new_state.time);

        // Advance time
        new_state.time = next_time;

        // Apply effects scheduled for this time
        let mut remaining_effects = Vec::new();
        for effect in new_state.scheduled_effects {
            if effect.time <= next_time {
                // Apply effect
                // TODO: Apply scheduled effect to state
                // For now, just consume the effect
            } else {
                remaining_effects.push(effect);
            }
        }
        
        new_state.scheduled_effects = remaining_effects;
        new_state
    }

    fn extract_plan(&self, goal_node: &SearchNode) -> SearchResult {
        let mut plan = Vec::new();
        let mut current = Some(goal_node);

        while let Some(node) = current {
            if let Some(action_idx) = node.action_idx {
                plan.push((action_idx, node.state.time));
            }
            current = node.parent.as_ref().map(|p| p.as_ref());
        }

        plan.reverse();
        
        SearchResult::Solution(Plan {
            actions: plan.into_iter().map(|(idx, _)| idx).collect(),
            cost: goal_node.g_value,
        })
    }
}