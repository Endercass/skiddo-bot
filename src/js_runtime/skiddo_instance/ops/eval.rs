use deno_core::JsRuntime;
use ipc_channel::platform::OsIpcSender;

use crate::{Message, Op};

pub fn eval(
    msg: Message,
    code: String,
    (runtime, controller_tx): (&mut JsRuntime, OsIpcSender),
) -> bool {
    let next = msg.next(Op::EvalComplete(format!(
        "{:#?}",
        runtime
            .execute_script("skiddo.runtime.eval", code.into())
            .unwrap()
    )));
    controller_tx
        .clone()
        .send(&bincode::serialize(&next).unwrap()[..], vec![], vec![])
        .unwrap();

    true
}
