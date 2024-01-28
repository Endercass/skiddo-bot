use poise::serenity_prelude as serenity;

use skiddo_bot::{
    bot::{commands, Data},
    Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = std::env::var("TOKEN").expect("Expected a token in the environment");
    let intents = serenity::GatewayIntents::MESSAGE_CONTENT;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: commands::commands(),
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;

    Ok(client?.start().await?)
}
