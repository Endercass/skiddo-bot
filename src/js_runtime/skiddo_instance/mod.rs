use std::process;

use scorched::{logf, LogData, LogImportance};

pub async fn run(socket: String, skiddo_file: String) -> Result<(), crate::Error> {
    let prefix = format!("Skiddo \"{}\" (PID {}):", &skiddo_file, process::id());
    scorched::set_log_prefix(&prefix);

    logf!(Info, "Booting up Skiddo instance...");

    let (controller_tx, worker_tx, worker_rx) = {
        let (worker_tx, worker_rx) = ipc_channel::platform::channel()?;
        (
            ipc_channel::platform::OsIpcSender::connect(socket.to_string())?,
            worker_tx,
            std::sync::Arc::new(std::sync::Mutex::new(worker_rx)),
        )
    };

    controller_tx
        .send(
            &[],
            vec![ipc_channel::platform::OsIpcChannel::Sender(
                worker_tx.clone(),
            )],
            vec![],
        )
        .unwrap();

    logf!(Info, "Skiddo instance booted up.");

    // let mut runtime = JsRuntime::new(RuntimeOptions {
    //     extensions: vec![],
    //     ..Default::default()
    // });

    Ok(())
}
