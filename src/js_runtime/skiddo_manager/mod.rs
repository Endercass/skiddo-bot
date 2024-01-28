use std::process;

use scorched::{logf, LogData, LogImportance};

pub async fn run() -> Result<(), crate::Error> {
    let prefix = format!("SkiddoManger (PID {}):", process::id());
    scorched::set_log_prefix(&prefix);

    logf!(Info, "Booting up SkiddoManager...");
    Ok(())
}
