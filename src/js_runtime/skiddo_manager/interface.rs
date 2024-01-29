use std::{
    collections::HashMap,
    env,
    process::{self, Stdio},
    sync::{Arc, Mutex, MutexGuard},
};

use ipc_channel::platform::{OsIpcOneShotServer, OsIpcReceiver, OsIpcSender};

use crate::{Message, Op};

pub struct SkiddoManagerInterface {
    pub skiddo_instances: HashMap<String, SkiddoInstance>,
}

#[zbus::dbus_interface(name = "me.endercass.skiddo_manager.SkiddoManagerInterface")]
impl SkiddoManagerInterface {
    fn spawn(&mut self, skiddo_file: String) {
        self.skiddo_instances
            .insert(skiddo_file.clone(), SkiddoInstance::spawn(skiddo_file));
    }

    fn eval(&self, skiddo_file: String, code: String) -> String {
        format!("TODO: Implement. {{{} {}}}", skiddo_file, code)
    }

    fn instances(&self) -> Vec<String> {
        self.skiddo_instances.keys().cloned().collect()
    }
}

#[derive(Clone)]
pub struct SkiddoInstance {
    pub skiddo_file: String,
    pub proc: Arc<Mutex<process::Child>>,
    tx: Arc<Mutex<OsIpcSender>>,
    rx: Arc<Mutex<OsIpcReceiver>>,
    message: Arc<Mutex<Option<Message>>>,
}

impl SkiddoInstance {
    pub fn spawn(skiddo_file: String) -> Self {
        let (server, name) = OsIpcOneShotServer::new().unwrap();
        let proc = Arc::new(Mutex::new(
            process::Command::new(env::current_exe().unwrap())
                .arg(format!("--worker={}", name))
                .arg(format!("--skiddo-file={}", skiddo_file))
                .stdin(Stdio::null())
                .stdout(Stdio::inherit())
                .stderr(Stdio::inherit())
                .spawn()
                .expect("failed to execute server process"),
        ));

        let (tx, rx) = {
            let (rx, _, mut received_channels, _) = server.accept().unwrap();
            let tx = received_channels[0].to_sender();
            (Arc::new(Mutex::new(tx)), Arc::new(Mutex::new(rx)))
        };

        let message = Arc::new(Mutex::new(None));

        Self {
            skiddo_file,
            proc,
            tx,
            rx,
            message,
        }
    }

    pub fn tx(&self) -> Option<OsIpcSender> {
        if self.message.lock().unwrap().is_some() {
            return Some(self.tx.lock().unwrap().clone());
        }
        None
    }

    pub fn rx(&self) -> Option<MutexGuard<'_, OsIpcReceiver>> {
        if self.message.lock().unwrap().is_some() {
            return Some(self.rx.lock().unwrap());
        }
        None
    }

    pub fn next(&self, op: Op) -> Message {
        let mut message = self.message.lock().unwrap();
        let next = message.clone().unwrap().next(op);
        *message = Some(next.clone());
        next
    }

    pub fn init(&self) -> Message {
        let mut message = self.message.lock().unwrap();
        let next = Message::init();
        *message = Some(next.clone());
        next
    }
}
