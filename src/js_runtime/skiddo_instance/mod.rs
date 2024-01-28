use std::process;

use scorched::{logf, LogData, LogImportance};

pub async fn run(skiddo_file: String) -> Result<(), crate::Error> {
    let prefix = format!("Skiddo \"{}\" (PID {}):", &skiddo_file, process::id());
    scorched::set_log_prefix(&prefix);

    logf!(Info, "Booting up Skiddo instance...");
    Ok(())
}
