// f:\common\Source_Code\TemporalFastDownward\rust\src\temporal_planner\state_space.rs
use super::temporal_task::{TemporalTask, State, TemporalAction};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct TemporalState {
    pub classical_state: State,
    pub scheduled_effects: Vec<ScheduledEffect>,
    pub time: f64,
}

#[derive(Debug, Clone)]
pub struct ScheduledEffect {
    pub time: f64,
    pub effect: super::temporal_task::Effect,
    pub action_id: usize,
}

pub struct StateSpace {
    task: TemporalTask,
    state_registry: HashMap<State, usize>,
}

impl StateSpace {
    pub fn new(task: TemporalTask) -> Self {
        Self {
            task,
            state_registry: HashMap::new(),
        }
    }

    pub fn get_applicable_actions(&self, state: &TemporalState) -> Vec<(usize, f64)> {
        let mut applicable = Vec::new();
        
        for (idx, action) in self.task.actions.iter().enumerate() {
            if self.is_applicable(action, state) {
                applicable.push((idx, state.time));
            }
        }
        
        applicable
    }

    fn is_applicable(&self, action: &TemporalAction, state: &TemporalState) -> bool {
        // Check start conditions
        for condition in &action.conditions_start {
            if !self.check_condition(condition, &state.classical_state) {
                return false;
            }
        }
        
        // Check mutex constraints
        // ...existing code...
        
        true
    }

    fn check_condition(&self, _condition: &super::temporal_task::Condition, _state: &State) -> bool {
        // Check if condition is satisfied in state
        // TODO: Implement condition checking
        true  // Assume all conditions are satisfied for now
    }

    pub fn apply_action(&self, state: &TemporalState, action_idx: usize, start_time: f64) -> TemporalState {
        let action = &self.task.actions[action_idx];
        let mut new_state = state.clone();
        
        // Apply start effects immediately
        for effect in &action.effects_start {
            self.apply_effect(&mut new_state.classical_state, effect);
        }
        
        // Schedule end effects
        for effect in &action.effects_end {
            new_state.scheduled_effects.push(ScheduledEffect {
                time: start_time + action.duration,
                effect: effect.clone(),
                action_id: action_idx,
            });
        }
        
        new_state
    }

    fn apply_effect(&self, _state: &mut State, _effect: &super::temporal_task::Effect) {
        // Apply effect to state
        // TODO: Implement effect application
    }
}