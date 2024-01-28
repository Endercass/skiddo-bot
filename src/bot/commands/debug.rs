use crate::bot::Context;
use crate::bot::Data;
use crate::Error;

pub fn debug_commands() -> Vec<poise::Command<Data, Error>> {
    vec![register()]
}

#[poise::command(prefix_command)]
pub async fn register(ctx: Context<'_>) -> Result<(), Error> {
    poise::builtins::register_application_commands_buttons(ctx).await?;
    Ok(())
}
