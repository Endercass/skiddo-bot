use std::env;

use skiddo_bot::{
    js_runtime::{skiddo_instance, skiddo_manager},
    Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = env::args().skip(1).collect::<Vec<_>>();

    if let Some(arg) = args
        .iter()
        .filter(|arg| arg.starts_with("--worker="))
        .map(|arg| arg.trim_start_matches("--worker="))
        .filter(|s| !s.is_empty())
        .last()
    {
        return skiddo_instance::run(arg.to_string()).await;
    } else {
        return skiddo_manager::run().await;
    }
}
