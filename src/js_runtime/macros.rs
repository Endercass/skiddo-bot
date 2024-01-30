#[macro_export]
macro_rules! message_handler {
    ($rx:expr, $($operation:pat => $body:expr),*) => {{
        while let Ok(recv) = $rx.recv() {
            if let Ok(msg) = bincode::deserialize::<crate::Message>(&recv.0) {
                let op = msg.op().clone();

                match op {
                    $(
                        $operation => {
                            if !$body(msg) {
                                break;
                            }
                        },
                    )*
                    _ => {}
                }
            }
        }
    }}
}
