pub mod interface;

use scorched::{logf, LogData, LogImportance};

pub async fn run() -> Result<(), crate::Error> {
    let prefix = format!("SkiddoManager (PID {}):", std::process::id());
    scorched::set_log_prefix(&prefix);

    logf!(Info, "Booting up SkiddoManager...");

    let skiddo_interface = interface::SkiddoManagerInterface {
        skiddo_instances: Default::default(),
    };

    let _conn = zbus::ConnectionBuilder::session()?
        .name("me.endercass.skiddo_manager")?
        .serve_at("/me/endercass/skiddo_manager", skiddo_interface)?
        .build()
        .await?;

    std::future::pending::<()>().await;

    Ok(())
}
