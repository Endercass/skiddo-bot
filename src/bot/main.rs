use std::process;

use poise::serenity_prelude as serenity;

use skiddo_bot::{
    bot::{commands, events, Data},
    Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = std::env::var("TOKEN").expect("Expected a token in the environment");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::MESSAGE_CONTENT;

    let prefix = format!("SkiddoBot (PID {}):", process::id());
    scorched::set_log_prefix(&prefix);

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::commands(),
            event_handler: events::event_handler,
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {
                    zbus_conn: zbus::Connection::session().await?,
                })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    Ok(client?.start().await?)
}
