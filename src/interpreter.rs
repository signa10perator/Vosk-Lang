use crate::ast::*;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub enum RuntimeState {
    Unknown,
    Resolved,
    Decaying(f64),
    Corrupted,
    Value(f64),
    Str(String),
}

pub struct Interpreter {
    pub bindings: HashMap<String, RuntimeState>,
    pub corrupted: bool,
}

impl Interpreter {
    pub fn new() -> Self {
        Interpreter {
            bindings: HashMap::new(),
            corrupted: false,
        }
    }

    pub fn run_program(&mut self, program: &Program) {
        for context in &program.contexts {
            println!("~ {} {{", context.name);
            self.run_context(context);
            println!("}}");

            if self.corrupted {
                println!("[CONTEXT CORRUPTED]");
            }
        }
    }

    fn run_context(&mut self, context: &Context) {
        for stmt in &context.body {
            self.run_stmt(stmt);
        }
    }

    fn run_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Binding { name, state } => {
                let runtime = self.state_to_runtime(state);
                println!("  {} :: {:?}", name, runtime);
                self.bindings.insert(name.clone(), runtime);
            }

            Stmt::Constraint { target, condition } => {
                let expected = self.state_to_runtime(condition);
                let actual = self.bindings.get(target).cloned();

                let satisfied = match (&actual, &expected) {
                    (Some(RuntimeState::Resolved), RuntimeState::Resolved) => true,
                    (Some(RuntimeState::Unknown), RuntimeState::Unknown) => true,
                    (Some(RuntimeState::Corrupted), RuntimeState::Corrupted) => true,
                    _ => false,
                };

                if satisfied {
                    println!("  ! {} :: satisfied", target);
                } else {
                    println!("  ! {} :: VIOLATED — context corrupting", target);
                    self.corrupted = true;
                }
            }

            Stmt::Observe { target, condition, .. } => {
                let watch = self.state_to_runtime(condition);
                let actual = self.bindings.get(target).cloned();

                let triggered = match (&actual, &watch) {
                    (Some(RuntimeState::Corrupted), RuntimeState::Corrupted) => true,
                    (Some(RuntimeState::Unknown), RuntimeState::Unknown) => true,
                    (Some(RuntimeState::Resolved), RuntimeState::Resolved) => true,
                    _ => false,
                };

                if triggered {
                    println!("  @ {} :: condition met — transmission fired", target);
                } else {
                    println!("  @ {} :: watching", target);
                }
            }
        }
    }

    fn state_to_runtime(&self, state: &State) -> RuntimeState {
        match state {
            State::Unknown => RuntimeState::Unknown,
            State::Resolved => RuntimeState::Resolved,
            State::Decaying => RuntimeState::Decaying(1.0),
            State::Corrupted => RuntimeState::Corrupted,
            State::Value(n) => RuntimeState::Value(*n),
            State::Str(s) => RuntimeState::Str(s.clone()),
        }
    }
}
