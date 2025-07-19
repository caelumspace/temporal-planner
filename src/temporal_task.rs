// f:\common\Source_Code\TemporalFastDownward\rust\src\temporal_planner\temporal_task.rs
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use regex::Regex;

#[derive(Debug, Clone)]
pub struct TemporalAction {
    pub name: String,
    pub duration: f64,
    pub conditions_start: Vec<Condition>,
    pub conditions_over_all: Vec<Condition>,
    pub conditions_end: Vec<Condition>,
    pub effects_start: Vec<Effect>,
    pub effects_end: Vec<Effect>,
}

#[derive(Debug, Clone)]
pub struct Condition {
    pub predicate: String,
    pub args: Vec<String>,
    pub is_negative: bool,
}

#[derive(Debug, Clone)]
pub struct Effect {
    pub predicate: String,
    pub args: Vec<String>,
    pub is_delete: bool,
}

#[derive(Debug, Clone)]
pub struct TemporalTask {
    pub initial_state: State,
    pub goal_conditions: Vec<Condition>,
    pub actions: Vec<TemporalAction>,
    pub mutex_groups: Vec<MutexGroup>,
}

#[derive(Debug, Clone)]
pub struct State {
    pub facts: Vec<bool>,
    pub numeric_values: HashMap<String, f64>,
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.facts == other.facts &&
        self.numeric_values.len() == other.numeric_values.len() &&
        self.numeric_values.iter().all(|(k, v)| {
            other.numeric_values.get(k).map_or(false, |ov| (v - ov).abs() < f64::EPSILON)
        })
    }
}

impl Eq for State {}

impl Hash for State {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.facts.hash(state);
        for (k, v) in &self.numeric_values {
            k.hash(state);
            // Use a simple approximation for f64 hashing
            let int_val = (v * 1000000.0).round() as i64;
            int_val.hash(state);
        }
    }
}

#[derive(Debug, Clone)]
pub struct MutexGroup {
    pub facts: Vec<usize>,
}

// PDDL parsing structures
#[derive(Debug, Clone)]
struct PDDLDomain {
    name: String,
    requirements: Vec<String>,
    types: Vec<String>,
    predicates: Vec<PDDLPredicate>,
    actions: Vec<PDDLAction>,
}

#[derive(Debug, Clone)]
struct PDDLPredicate {
    name: String,
    parameters: Vec<PDDLParameter>,
}

#[derive(Debug, Clone)]
struct PDDLParameter {
    name: String,
    type_name: Option<String>,
}

#[derive(Debug, Clone)]
struct PDDLAction {
    name: String,
    parameters: Vec<PDDLParameter>,
    precondition: Option<PDDLFormula>,
    effect: Option<PDDLFormula>,
    duration: Option<PDDLDuration>,
    is_durative: bool,
}

#[derive(Debug, Clone)]
enum PDDLDuration {
    Fixed(f64),
    Variable(String),
    Expression(Box<PDDLFormula>),
}

#[derive(Debug, Clone)]
enum PDDLFormula {
    Predicate {
        name: String,
        args: Vec<String>,
        negated: bool,
    },
    And(Vec<PDDLFormula>),
    Or(Vec<PDDLFormula>),
    Not(Box<PDDLFormula>),
    AtStart(Box<PDDLFormula>),
    AtEnd(Box<PDDLFormula>),
    OverAll(Box<PDDLFormula>),
}

impl TemporalTask {
    pub fn new() -> Self {
        Self {
            initial_state: State {
                facts: Vec::new(),
                numeric_values: HashMap::new(),
            },
            goal_conditions: Vec::new(),
            actions: Vec::new(),
            mutex_groups: Vec::new(),
        }
    }

    pub fn from_pddl(domain_content: &str, problem_content: &str) -> Self {
        // Parse the PDDL domain and problem files
        let domain = Self::parse_pddl_domain(domain_content);
        let mut task = Self::new();
        
        // Convert PDDL actions to temporal actions
        task.actions = Self::convert_pddl_actions(&domain.actions, &domain.predicates);
        
        // Parse problem file for initial state and goals
        let (initial_state, goal_conditions) = Self::parse_pddl_problem(problem_content, &domain.predicates);
        task.initial_state = initial_state;
        task.goal_conditions = goal_conditions;
        
        task
    }
    
    fn parse_pddl_domain(content: &str) -> PDDLDomain {
        let mut domain = PDDLDomain {
            name: String::new(),
            requirements: Vec::new(),
            types: Vec::new(),
            predicates: Vec::new(),
            actions: Vec::new(),
        };
        
        // Remove comments and normalize whitespace
        let cleaned_content = Self::clean_pddl_content(content);
        
        // Parse domain name
        if let Some(name) = Self::extract_domain_name(&cleaned_content) {
            domain.name = name;
        }
        
        // Parse requirements
        domain.requirements = Self::extract_requirements(&cleaned_content);
        
        // Parse types
        domain.types = Self::extract_types(&cleaned_content);
        
        // Parse predicates
        domain.predicates = Self::extract_predicates(&cleaned_content);
        
        // Parse actions
        domain.actions = Self::extract_actions(&cleaned_content);
        
        domain
    }
    
    fn clean_pddl_content(content: &str) -> String {
        // Remove comments (lines starting with ;)
        let comment_regex = Regex::new(r";.*$").unwrap();
        let lines: Vec<&str> = content.lines().collect();
        let cleaned_lines: Vec<String> = lines
            .iter()
            .map(|line| comment_regex.replace_all(line, "").to_string())
            .collect();
        
        // Join lines and normalize whitespace
        cleaned_lines.join(" ")
            .split_whitespace()
            .collect::<Vec<&str>>()
            .join(" ")
    }
    
    fn extract_domain_name(content: &str) -> Option<String> {
        let regex = Regex::new(r"\(define\s+\(domain\s+([^)]+)\)").unwrap();
        regex.captures(content)
            .and_then(|caps| caps.get(1))
            .map(|m| m.as_str().to_string())
    }
    
    fn extract_requirements(content: &str) -> Vec<String> {
        let regex = Regex::new(r"\(:requirements\s+([^)]+)\)").unwrap();
        if let Some(caps) = regex.captures(content) {
            if let Some(reqs_str) = caps.get(1) {
                return reqs_str.as_str()
                    .split_whitespace()
                    .map(|s| s.to_string())
                    .collect();
            }
        }
        Vec::new()
    }
    
    fn extract_types(content: &str) -> Vec<String> {
        let regex = Regex::new(r"\(:types\s+([^)]+)\)").unwrap();
        if let Some(caps) = regex.captures(content) {
            if let Some(types_str) = caps.get(1) {
                return types_str.as_str()
                    .split_whitespace()
                    .filter(|s| !s.starts_with('-'))
                    .map(|s| s.to_string())
                    .collect();
            }
        }
        Vec::new()
    }
    
    fn extract_predicates(content: &str) -> Vec<PDDLPredicate> {
        let mut predicates = Vec::new();
        let predicate_regex = Regex::new(r"\(([a-zA-Z0-9_-]+)([^)]*)\)").unwrap();
        
        // Find the predicates section
        if let Some(start) = content.find("(:predicates") {
            let after_predicates = &content[start..];
            let mut depth = 0;
            let mut pred_section = String::new();
            
            for ch in after_predicates.chars() {
                pred_section.push(ch);
                match ch {
                    '(' => depth += 1,
                    ')' => {
                        depth -= 1;
                        if depth == 0 {
                            break;
                        }
                    }
                    _ => {}
                }
            }
            
            // Extract individual predicates
            for cap in predicate_regex.captures_iter(&pred_section) {
                if let Some(pred_name) = cap.get(1) {
                    let name = pred_name.as_str().to_string();
                    if name != "predicates" {
                        let params_str = cap.get(2).map(|m| m.as_str()).unwrap_or("");
                        let parameters = Self::parse_parameters(params_str);
                        
                        predicates.push(PDDLPredicate {
                            name,
                            parameters,
                        });
                    }
                }
            }
        }
        
        predicates
    }
    
    fn extract_actions(content: &str) -> Vec<PDDLAction> {
        let mut actions = Vec::new();
        let action_regex = Regex::new(r"\(:(action|durative-action)\s+([a-zA-Z0-9_-]+)").unwrap();
        
        for cap in action_regex.captures_iter(content) {
            if let Some(action_name) = cap.get(2) {
                let action_type = cap.get(1).unwrap().as_str();
                let name = action_name.as_str().to_string();
                let action_start = cap.get(0).unwrap().start();
                
                // Find the complete action definition
                let action_content = Self::extract_balanced_expression(&content[action_start..]);
                
                // Parse action components
                let parameters = Self::extract_action_parameters(&action_content);
                let (precondition, effect, duration, is_durative) = if action_type == "durative-action" {
                    // Parse durative action
                    let condition = Self::extract_action_condition(&action_content);
                    let effect = Self::extract_action_effect(&action_content);
                    let duration = Self::extract_action_duration(&action_content);
                    (condition, effect, Some(duration), true)
                } else {
                    // Parse regular action
                    let precondition = Self::extract_action_precondition(&action_content);
                    let effect = Self::extract_action_effect(&action_content);
                    (precondition, effect, None, false)
                };
                
                actions.push(PDDLAction {
                    name,
                    parameters,
                    precondition,
                    effect,
                    duration,
                    is_durative,
                });
            }
        }
        
        actions
    }
    
    fn extract_balanced_expression(content: &str) -> String {
        let mut result = String::new();
        let mut depth = 0;
        let mut started = false;
        
        for ch in content.chars() {
            if ch == '(' {
                depth += 1;
                started = true;
            } else if ch == ')' {
                depth -= 1;
            }
            
            if started {
                result.push(ch);
                if depth == 0 {
                    break;
                }
            }
        }
        
        result
    }
    
    fn parse_parameters(params_str: &str) -> Vec<PDDLParameter> {
        let mut parameters = Vec::new();
        let tokens: Vec<&str> = params_str.split_whitespace().collect();
        let mut i = 0;
        
        while i < tokens.len() {
            let token = tokens[i];
            if token.starts_with('?') {
                let mut param = PDDLParameter {
                    name: token.to_string(),
                    type_name: None,
                };
                
                // Check if there's a type specification
                if i + 2 < tokens.len() && tokens[i + 1] == "-" {
                    param.type_name = Some(tokens[i + 2].to_string());
                    i += 3;
                } else {
                    i += 1;
                }
                
                parameters.push(param);
            } else {
                i += 1;
            }
        }
        
        parameters
    }
    
    fn extract_action_parameters(action_content: &str) -> Vec<PDDLParameter> {
        let regex = Regex::new(r":parameters\s+\(([^)]*)\)").unwrap();
        if let Some(caps) = regex.captures(action_content) {
            if let Some(params_str) = caps.get(1) {
                return Self::parse_parameters(params_str.as_str());
            }
        }
        Vec::new()
    }
    
    fn extract_action_precondition(action_content: &str) -> Option<PDDLFormula> {
        if let Some(start) = action_content.find(":precondition") {
            let after_precond = &action_content[start + ":precondition".len()..];
            let formula_str = Self::extract_balanced_expression(after_precond.trim_start());
            return Self::parse_formula(&formula_str);
        }
        None
    }
    
    fn extract_action_condition(action_content: &str) -> Option<PDDLFormula> {
        if let Some(start) = action_content.find(":condition") {
            let after_cond = &action_content[start + ":condition".len()..];
            let formula_str = Self::extract_balanced_expression(after_cond.trim_start());
            return Self::parse_formula(&formula_str);
        }
        None
    }
    
    fn extract_action_duration(action_content: &str) -> PDDLDuration {
        if let Some(start) = action_content.find(":duration") {
            let after_duration = &action_content[start + ":duration".len()..];
            let duration_str = Self::extract_balanced_expression(after_duration.trim_start());
            
            // Parse duration expression
            let cleaned = duration_str.trim().trim_start_matches('(').trim_end_matches(')');
            
            if cleaned.starts_with("= ?duration") {
                // Extract the duration value
                let tokens: Vec<&str> = cleaned.split_whitespace().collect();
                if tokens.len() >= 3 {
                    if let Ok(duration_val) = tokens[2].parse::<f64>() {
                        return PDDLDuration::Fixed(duration_val);
                    }
                }
            }
            
            // Default to 1.0 if parsing fails
            PDDLDuration::Fixed(1.0)
        } else {
            PDDLDuration::Fixed(1.0)
        }
    }
    
    fn extract_action_effect(action_content: &str) -> Option<PDDLFormula> {
        if let Some(start) = action_content.find(":effect") {
            let after_effect = &action_content[start + ":effect".len()..];
            let formula_str = Self::extract_balanced_expression(after_effect.trim_start());
            return Self::parse_formula(&formula_str);
        }
        None
    }
    
    fn parse_formula(formula_str: &str) -> Option<PDDLFormula> {
        let trimmed = formula_str.trim();
        
        if !trimmed.starts_with('(') || !trimmed.ends_with(')') {
            return None;
        }
        
        let inner = &trimmed[1..trimmed.len()-1];
        let tokens = Self::tokenize_formula(inner);
        
        if tokens.is_empty() {
            return None;
        }
        
        match tokens[0].as_str() {
            "and" => {
                let mut formulas = Vec::new();
                let remaining_tokens = &tokens[1..];
                let sub_formulas = Self::extract_sub_formulas(remaining_tokens);
                
                for sub_formula in sub_formulas {
                    if let Some(parsed) = Self::parse_formula(&sub_formula) {
                        formulas.push(parsed);
                    }
                }
                Some(PDDLFormula::And(formulas))
            },
            "or" => {
                let mut formulas = Vec::new();
                let remaining_tokens = &tokens[1..];
                let sub_formulas = Self::extract_sub_formulas(remaining_tokens);
                
                for sub_formula in sub_formulas {
                    if let Some(parsed) = Self::parse_formula(&sub_formula) {
                        formulas.push(parsed);
                    }
                }
                Some(PDDLFormula::Or(formulas))
            },
            "not" => {
                let remaining_tokens = &tokens[1..];
                if !remaining_tokens.is_empty() {
                    let sub_formula = remaining_tokens.join(" ");
                    if let Some(parsed) = Self::parse_formula(&format!("({})", sub_formula)) {
                        Some(PDDLFormula::Not(Box::new(parsed)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            "at" => {
                // Handle temporal operators: (at start ...), (at end ...)
                if tokens.len() >= 2 {
                    match tokens[1].as_str() {
                        "start" => {
                            let remaining_tokens = &tokens[2..];
                            let sub_formula = remaining_tokens.join(" ");
                            if let Some(parsed) = Self::parse_formula(&format!("({})", sub_formula)) {
                                Some(PDDLFormula::AtStart(Box::new(parsed)))
                            } else {
                                None
                            }
                        },
                        "end" => {
                            let remaining_tokens = &tokens[2..];
                            let sub_formula = remaining_tokens.join(" ");
                            if let Some(parsed) = Self::parse_formula(&format!("({})", sub_formula)) {
                                Some(PDDLFormula::AtEnd(Box::new(parsed)))
                            } else {
                                None
                            }
                        },
                        _ => None
                    }
                } else {
                    None
                }
            },
            "over" => {
                // Handle temporal operator: (over all ...)
                if tokens.len() >= 2 && tokens[1] == "all" {
                    let remaining_tokens = &tokens[2..];
                    let sub_formula = remaining_tokens.join(" ");
                    if let Some(parsed) = Self::parse_formula(&format!("({})", sub_formula)) {
                        Some(PDDLFormula::OverAll(Box::new(parsed)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            _ => {
                // Simple predicate
                let name = tokens[0].clone();
                let args = tokens[1..].to_vec();
                Some(PDDLFormula::Predicate {
                    name,
                    args,
                    negated: false,
                })
            }
        }
    }
    
    fn tokenize_formula(formula: &str) -> Vec<String> {
        let mut tokens = Vec::new();
        let mut current_token = String::new();
        let mut depth = 0;
        let mut in_token = false;
        
        for ch in formula.chars() {
            match ch {
                '(' => {
                    if depth > 0 || in_token {
                        current_token.push(ch);
                    }
                    depth += 1;
                },
                ')' => {
                    depth -= 1;
                    if depth > 0 || in_token {
                        current_token.push(ch);
                    }
                    if depth == 0 && in_token {
                        tokens.push(format!("({})", current_token));
                        current_token.clear();
                        in_token = false;
                    }
                },
                ' ' | '\t' | '\n' | '\r' => {
                    if depth > 0 {
                        current_token.push(ch);
                    } else if !current_token.is_empty() {
                        tokens.push(current_token.clone());
                        current_token.clear();
                        in_token = false;
                    }
                },
                _ => {
                    current_token.push(ch);
                    if depth == 0 {
                        in_token = true;
                    }
                }
            }
        }
        
        if !current_token.is_empty() {
            if depth > 0 {
                tokens.push(format!("({})", current_token));
            } else {
                tokens.push(current_token);
            }
        }
        
        tokens
    }
    
    fn extract_sub_formulas(tokens: &[String]) -> Vec<String> {
        let mut sub_formulas = Vec::new();
        let mut current_formula = String::new();
        let mut depth = 0;
        
        for token in tokens {
            if token.starts_with('(') {
                if depth > 0 {
                    current_formula.push(' ');
                    current_formula.push_str(token);
                } else {
                    current_formula = token.clone();
                }
                depth += token.chars().filter(|&c| c == '(').count();
                depth -= token.chars().filter(|&c| c == ')').count();
                
                if depth == 0 {
                    sub_formulas.push(current_formula.clone());
                    current_formula.clear();
                }
            } else if depth > 0 {
                current_formula.push(' ');
                current_formula.push_str(token);
            } else {
                // Single token predicate
                sub_formulas.push(format!("({})", token));
            }
        }
        
        if !current_formula.is_empty() {
            sub_formulas.push(current_formula);
        }
        
        sub_formulas
    }
    
    fn convert_pddl_actions(pddl_actions: &[PDDLAction], _predicates: &[PDDLPredicate]) -> Vec<TemporalAction> {
        pddl_actions.iter().map(|action| {
            let duration = match &action.duration {
                Some(PDDLDuration::Fixed(d)) => *d,
                _ => 1.0, // Default duration
            };
            
            if action.is_durative {
                // For durative actions, separate conditions and effects by time
                let (conditions_start, conditions_over_all, conditions_end) = 
                    Self::extract_temporal_conditions(&action.precondition);
                let (effects_start, effects_end) = 
                    Self::extract_temporal_effects(&action.effect);
                
                TemporalAction {
                    name: action.name.clone(),
                    duration,
                    conditions_start,
                    conditions_over_all,
                    conditions_end,
                    effects_start,
                    effects_end,
                }
            } else {
                // Regular actions - all conditions at start, all effects at end
                TemporalAction {
                    name: action.name.clone(),
                    duration,
                    conditions_start: Self::extract_conditions_from_formula(&action.precondition),
                    conditions_over_all: Vec::new(),
                    conditions_end: Vec::new(),
                    effects_start: Vec::new(),
                    effects_end: Self::extract_effects_from_formula(&action.effect),
                }
            }
        }).collect()
    }
    
    fn extract_temporal_conditions(formula: &Option<PDDLFormula>) -> (Vec<Condition>, Vec<Condition>, Vec<Condition>) {
        let mut conditions_start = Vec::new();
        let mut conditions_over_all = Vec::new();
        let mut conditions_end = Vec::new();
        
        if let Some(formula) = formula {
            Self::collect_temporal_conditions_recursive(formula, &mut conditions_start, &mut conditions_over_all, &mut conditions_end);
        }
        
        (conditions_start, conditions_over_all, conditions_end)
    }
    
    fn extract_temporal_effects(formula: &Option<PDDLFormula>) -> (Vec<Effect>, Vec<Effect>) {
        let mut effects_start = Vec::new();
        let mut effects_end = Vec::new();
        
        if let Some(formula) = formula {
            Self::collect_temporal_effects_recursive(formula, &mut effects_start, &mut effects_end);
        }
        
        (effects_start, effects_end)
    }
    
    fn collect_temporal_conditions_recursive(
        formula: &PDDLFormula, 
        conditions_start: &mut Vec<Condition>,
        conditions_over_all: &mut Vec<Condition>, 
        conditions_end: &mut Vec<Condition>
    ) {
        match formula {
            PDDLFormula::AtStart(inner) => {
                Self::collect_conditions_recursive(inner, conditions_start);
            },
            PDDLFormula::OverAll(inner) => {
                Self::collect_conditions_recursive(inner, conditions_over_all);
            },
            PDDLFormula::AtEnd(inner) => {
                Self::collect_conditions_recursive(inner, conditions_end);
            },
            PDDLFormula::And(formulas) => {
                for f in formulas {
                    Self::collect_temporal_conditions_recursive(f, conditions_start, conditions_over_all, conditions_end);
                }
            },
            _ => {
                // Default to start conditions for non-temporal formulas
                Self::collect_conditions_recursive(formula, conditions_start);
            }
        }
    }
    
    fn collect_temporal_effects_recursive(
        formula: &PDDLFormula, 
        effects_start: &mut Vec<Effect>,
        effects_end: &mut Vec<Effect>
    ) {
        match formula {
            PDDLFormula::AtStart(inner) => {
                Self::collect_effects_recursive(inner, effects_start);
            },
            PDDLFormula::AtEnd(inner) => {
                Self::collect_effects_recursive(inner, effects_end);
            },
            PDDLFormula::And(formulas) => {
                for f in formulas {
                    Self::collect_temporal_effects_recursive(f, effects_start, effects_end);
                }
            },
            _ => {
                // Default to end effects for non-temporal formulas
                Self::collect_effects_recursive(formula, effects_end);
            }
        }
    }
    
    fn extract_conditions_from_formula(formula: &Option<PDDLFormula>) -> Vec<Condition> {
        let mut conditions = Vec::new();
        
        if let Some(formula) = formula {
            Self::collect_conditions_recursive(formula, &mut conditions);
        }
        
        conditions
    }
    
    fn collect_conditions_recursive(formula: &PDDLFormula, conditions: &mut Vec<Condition>) {
        match formula {
            PDDLFormula::Predicate { name, args, negated } => {
                conditions.push(Condition {
                    predicate: name.clone(),
                    args: args.clone(),
                    is_negative: *negated,
                });
            },
            PDDLFormula::And(formulas) => {
                for f in formulas {
                    Self::collect_conditions_recursive(f, conditions);
                }
            },
            PDDLFormula::Or(formulas) => {
                // For simplicity, treat OR as AND for now
                for f in formulas {
                    Self::collect_conditions_recursive(f, conditions);
                }
            },
            PDDLFormula::Not(formula) => {
                if let PDDLFormula::Predicate { name, args, .. } = formula.as_ref() {
                    conditions.push(Condition {
                        predicate: name.clone(),
                        args: args.clone(),
                        is_negative: true,
                    });
                }
            },
            PDDLFormula::AtStart(formula) => {
                Self::collect_conditions_recursive(formula, conditions);
            },
            PDDLFormula::AtEnd(formula) => {
                Self::collect_conditions_recursive(formula, conditions);
            },
            PDDLFormula::OverAll(formula) => {
                Self::collect_conditions_recursive(formula, conditions);
            }
        }
    }
    
    fn extract_effects_from_formula(formula: &Option<PDDLFormula>) -> Vec<Effect> {
        let mut effects = Vec::new();
        
        if let Some(formula) = formula {
            Self::collect_effects_recursive(formula, &mut effects);
        }
        
        effects
    }
    
    fn collect_effects_recursive(formula: &PDDLFormula, effects: &mut Vec<Effect>) {
        match formula {
            PDDLFormula::Predicate { name, args, negated } => {
                effects.push(Effect {
                    predicate: name.clone(),
                    args: args.clone(),
                    is_delete: *negated,
                });
            },
            PDDLFormula::And(formulas) => {
                for f in formulas {
                    Self::collect_effects_recursive(f, effects);
                }
            },
            PDDLFormula::Not(formula) => {
                if let PDDLFormula::Predicate { name, args, .. } = formula.as_ref() {
                    effects.push(Effect {
                        predicate: name.clone(),
                        args: args.clone(),
                        is_delete: true,
                    });
                }
            },
            PDDLFormula::AtStart(formula) => {
                Self::collect_effects_recursive(formula, effects);
            },
            PDDLFormula::AtEnd(formula) => {
                Self::collect_effects_recursive(formula, effects);
            },
            _ => {}
        }
    }
    
    fn parse_pddl_problem(problem_content: &str, predicates: &[PDDLPredicate]) -> (State, Vec<Condition>) {
        let cleaned_content = Self::clean_pddl_content(problem_content);
        
        // Parse initial state
        let initial_state = Self::parse_initial_state(&cleaned_content, predicates);
        
        // Parse goal conditions
        let goal_conditions = Self::parse_goal_conditions(&cleaned_content);
        
        (initial_state, goal_conditions)
    }
    
    fn parse_initial_state(content: &str, predicates: &[PDDLPredicate]) -> State {
        let mut state = State {
            facts: vec![false; predicates.len()],
            numeric_values: HashMap::new(),
        };
        
        // Find the init section
        if let Some(start) = content.find("(:init") {
            let after_init = &content[start..];
            let init_section = Self::extract_balanced_expression(after_init);
            
            // Parse individual facts from the init section
            let fact_regex = Regex::new(r"\(([a-zA-Z0-9_-]+)([^)]*)\)").unwrap();
            
            for cap in fact_regex.captures_iter(&init_section) {
                if let Some(pred_name) = cap.get(1) {
                    let name = pred_name.as_str();
                    if name == "init" {
                        continue;
                    }
                    
                    let args_str = cap.get(2).map(|m| m.as_str()).unwrap_or("").trim();
                    let args: Vec<String> = if args_str.is_empty() {
                        Vec::new()
                    } else {
                        args_str.split_whitespace().map(|s| s.to_string()).collect()
                    };
                    
                    // Find the predicate index
                    if let Some(pred_index) = Self::find_predicate_index(predicates, name, &args) {
                        if pred_index < state.facts.len() {
                            state.facts[pred_index] = true;
                        }
                    }
                    
                    // Handle numeric values (= (function args) value)
                    if name == "=" && args.len() >= 2 {
                        if let Ok(value) = args[1].parse::<f64>() {
                            state.numeric_values.insert(args[0].clone(), value);
                        }
                    }
                }
            }
        }
        
        state
    }
    
    fn parse_goal_conditions(content: &str) -> Vec<Condition> {
        let mut goal_conditions = Vec::new();
        
        // Find the goal section
        if let Some(start) = content.find("(:goal") {
            let after_goal = &content[start + 6..];
            let goal_section = Self::extract_balanced_expression(after_goal.trim_start());
            
            // Parse the goal formula
            if let Some(formula) = Self::parse_formula(&goal_section) {
                Self::collect_conditions_recursive(&formula, &mut goal_conditions);
            }
        }
        
        goal_conditions
    }
    
    fn find_predicate_index(predicates: &[PDDLPredicate], name: &str, args: &[String]) -> Option<usize> {
        for (index, predicate) in predicates.iter().enumerate() {
            if predicate.name == name && predicate.parameters.len() == args.len() {
                return Some(index);
            }
        }
        None
    }
}