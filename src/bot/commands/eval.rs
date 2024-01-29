use crate::bot::proxy::SkiddoManagerInterfaceProxy;
use crate::bot::Context;
use crate::Error;

#[poise::command(slash_command)]
pub async fn eval(
    ctx: Context<'_>,
    #[description = "Code to evaluate"] code: String,
) -> Result<(), Error> {
    ctx.reply(format!("```js\nskiddo.eval(\"{}\")```", code))
        .await?;

    let proxy = SkiddoManagerInterfaceProxy::new(&ctx.data().zbus_conn)
        .await
        .unwrap();

    println!("{}", proxy.eval("/dev/sda".into(), code).await?);

    Ok(())
}
