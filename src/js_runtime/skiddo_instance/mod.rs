pub mod js;
pub mod ops;

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

    let mut runtime = deno_core::JsRuntime::new(deno_core::RuntimeOptions {
        extensions: vec![
            crate::js_runtime::skiddo_instance::js::ext::skiddo_internal::init_ops_and_esm(),
        ],
        ..Default::default()
    });

    logf!(Info, "Skiddo instance booted up.");

    crate::message_handler!(
        worker_rx.lock().unwrap(),
        crate::Op::Init => |msg: crate::Message | ops::init::init(msg, (&mut runtime, controller_tx.clone())),
        crate::Op::Eval(code) => |msg: crate::Message| ops::eval::eval(msg, code, (&mut runtime, controller_tx.clone())),
        crate::Op::Shutdown => |_msg: crate::Message| false
    );
    Ok(())
}
