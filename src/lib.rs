pub mod bot;
pub mod js_runtime;

pub type Error = Box<dyn std::error::Error + Send + Sync>;

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub enum Op {
    /// Initialize the skiddo
    Init,
    /// Initialization complete
    InitComplete,
    /// Evaluate a string
    Eval(String),
    /// Result of an evaluation
    EvalComplete(String),
    /// No operation (with optional message)
    ///
    /// Message MUST NOT be used to determine
    /// behavior
    Noop(Option<String>),
    /// Error (with optional message)
    ///
    /// Message is strongly encouraged
    /// and can be used to determine behavior
    Error(Option<String>),
    /// Shutdown the skiddo
    Shutdown,
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct Message {
    seq: u64,
    op: Op,
}

impl Message {
    pub fn init() -> Self {
        Message {
            seq: 0,
            op: Op::Init,
        }
    }
    pub fn op(&self) -> &Op {
        &self.op
    }
    pub fn next(&self, op: Op) -> Self {
        Message {
            seq: self.seq + 1,
            op,
        }
    }
}
