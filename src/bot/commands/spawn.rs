use crate::bot::proxy::SkiddoManagerInterfaceProxy;
use crate::bot::Context;
use crate::Error;

#[poise::command(slash_command)]
pub async fn spawn(
    ctx: Context<'_>,
    #[description = "Skiddo File"] skiddo_file: String,
) -> Result<(), Error> {
    ctx.reply(format!(
        "Booting up skiddo from Skiddo File {}",
        skiddo_file
    ))
    .await?;

    let mut proxy = SkiddoManagerInterfaceProxy::new(&ctx.data().zbus_conn)
        .await
        .unwrap();

    proxy.spawn(skiddo_file).await?;

    Ok(())
}
