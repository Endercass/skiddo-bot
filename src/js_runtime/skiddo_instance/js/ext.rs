use deno_core::extension;
use deno_core::op2;
use scorched::{logf, LogData, LogImportance};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
}

#[op2]
fn op_log(#[serde] level: LogLevel, #[string] msg: String) {
    match level {
        LogLevel::Debug => logf!(Debug, "JS: {}", msg),
        LogLevel::Info => logf!(Info, "JS: {}", msg),
        LogLevel::Warning => logf!(Warning, "JS: {}", msg),
        LogLevel::Error => logf!(Error, "JS: {}", msg),
    };
}

#[op2]
#[string]
fn op_get_skiddo_file() -> Option<String> {
    std::env::args()
        .skip(1)
        .collect::<Vec<_>>()
        .iter()
        .filter(|arg| arg.starts_with("--skiddo-file="))
        .map(|arg| arg.trim_start_matches("--skiddo-file=").to_string())
        .filter(|s| !s.is_empty())
        .last()
}

extension!(
    skiddo_internal,
    ops = [op_log, op_get_skiddo_file],
    esm_entry_point = "ext:skiddo_internal/ext.js",
    esm = [
        dir "src/js_runtime/skiddo_instance/js",
        "ext.js"
    ],
);
