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

impl RuntimeState {
    pub fn tick(&self) -> RuntimeState {
        match self {
            RuntimeState::Decaying(n) => {
                let next = n - 0.25;
                if next <= 0.0 {
                    RuntimeState::Corrupted
                } else {
                    RuntimeState::Decaying(next)
                }
            }
            other => other.clone(),
        }
    }
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
                self.bindings.clear();
                self.corrupted = false;
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
   
           println!("  -- decay tick --");
           self.tick_decay();
   
           for stmt in &context.body {
               if let Stmt::Observe { target, condition, transmit } = stmt {
                   let watch = self.state_to_runtime(condition);
                   let actual = self.bindings.get(target).cloned();
   
                   let triggered = match (&actual, &watch) {
                       (Some(RuntimeState::Corrupted), RuntimeState::Corrupted) => true,
                       _ => false,
                   };
   
                   if triggered {
                       println!("  @ {} :: condition met after decay", target);
                       if let Some(tx) = transmit {
                           match tx.scope {
                               TransmitScope::Emit => println!("  ~> emit \"{}\"", tx.message),
                               TransmitScope::Propagate => println!("  ~> * \"{}\"", tx.message),
                               TransmitScope::Escalate => println!("  ~> ^ \"{}\"", tx.message),
                               TransmitScope::Local => println!("  ~> \"{}\"", tx.message),
                           }
                       }
                   }
               }
           }
       }

    pub fn tick_decay(&mut self) {
            let mut corrupted = vec![];
    
            for (name, state) in self.bindings.iter_mut() {
                *state = state.tick();
                if let RuntimeState::Corrupted = state {
                    corrupted.push(name.clone());
                }
            }
    
            for name in corrupted {
                println!("  % {} :: decayed -> Corrupted", name);
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
                    (Some(RuntimeState::Str(_)), RuntimeState::Resolved) => true,
                    (Some(RuntimeState::Value(_)), RuntimeState::Resolved) => true,
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

Stmt::Observe { target, condition, transmit } => {
                let watch = self.state_to_runtime(condition);
                let actual = self.bindings.get(target).cloned();

                let triggered = match (&actual, &watch) {
                    (Some(RuntimeState::Corrupted), RuntimeState::Corrupted) => true,
                    (Some(RuntimeState::Unknown), RuntimeState::Unknown) => true,
                    (Some(RuntimeState::Resolved), RuntimeState::Resolved) => true,
                    _ => false,
                };

                if triggered {
                    println!("  @ {} :: condition met", target);
                    if let Some(tx) = transmit {
                        match tx.scope {
                            TransmitScope::Emit => {
                                println!("  ~> emit \"{}\"", tx.message);
                            }
                            TransmitScope::Propagate => {
                                println!("  ~> * \"{}\"", tx.message);
                            }
                            TransmitScope::Escalate => {
                                println!("  ~> ^ \"{}\"", tx.message);
                            }
                            TransmitScope::Local => {
                                println!("  ~> \"{}\"", tx.message);
                            }
                        }
                    }
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
            State::Decaying => RuntimeState::Decaying(0.25),
            State::DecayingValue(n) => RuntimeState::Decaying(*n),
            State::Corrupted => RuntimeState::Corrupted,
            State::Value(n) => RuntimeState::Value(*n),
            State::Str(s) => RuntimeState::Str(s.clone()),
        }
    }
}
