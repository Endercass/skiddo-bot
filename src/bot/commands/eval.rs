use crate::bot::Context;
use crate::Error;

#[poise::command(slash_command)]
pub async fn eval(ctx: Context<'_>) -> Result<(), Error> {
    ctx.reply("Hello, world!").await?;

    Ok(())
}
