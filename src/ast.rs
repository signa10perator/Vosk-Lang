#[derive(Debug, Clone)]
pub enum State {
    Unknown,
    Resolved,
    Decaying,
    DecayingValue(f64),
    Corrupted,
    Value(f64),
    Str(String),
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Binding {
        name: String,
        state: State,
    },
    Receiver {
            name: String,
            state: State,
    },
    Constraint {
        target: String,
        condition: State,
    },
    Observe {
        target: String,
        condition: State,
        transmit: Option<Box<Transmission>>,
    },
}

#[derive(Debug, Clone)]
pub struct Transmission {
    pub scope: TransmitScope,
    pub message: String,
}

#[derive(Debug, Clone)]
pub enum TransmitScope {
    Local,      // single target
    Propagate,  // * spread to all related contexts
    Escalate,   // ^ surface to parent context
    Emit,       // emit outward
}

#[derive(Debug, Clone)]
pub struct Context {
    pub name: String,
    pub body: Vec<Stmt>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub contexts: Vec<Context>,
}
