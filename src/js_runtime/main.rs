use std::env;

use skiddo_bot::{
    js_runtime::{skiddo_instance, skiddo_manager},
    Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    if let Some(socket) = args
        .iter()
        .filter(|arg| arg.starts_with("--worker="))
        .map(|arg| arg.trim_start_matches("--worker="))
        .filter(|s| !s.is_empty())
        .last()
    {
        if let Some(skiddo_file) = args
            .iter()
            .filter(|arg| arg.starts_with("--skiddo-file="))
            .map(|arg| arg.trim_start_matches("--skiddo-file="))
            .filter(|s| !s.is_empty())
            .last()
        {
            return skiddo_instance::run(socket.to_string(), skiddo_file.to_string()).await;
        }
        return skiddo_manager::run().await;
    } else {
        return skiddo_manager::run().await;
    }
}
