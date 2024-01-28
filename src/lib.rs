pub mod bot;
pub mod js_runtime;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
