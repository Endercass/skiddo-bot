pub mod ready;

pub fn event_handler<'a>(
    ctx: &'a poise::serenity_prelude::Context,
    event: &'a poise::serenity_prelude::FullEvent,
    _framework: poise::FrameworkContext<'a, crate::bot::Data, crate::Error>,
    _data: &'a crate::bot::Data,
) -> std::pin::Pin<
    Box<(dyn std::future::Future<Output = Result<(), crate::Error>> + std::marker::Send + 'a)>,
> {
    Box::pin(async move {
        match event {
            poise::serenity_prelude::FullEvent::Ready { data_about_bot } => {
                ready::ready_event(&data_about_bot, ctx).await;
                Ok(())
            }
            _ => Ok(()),
        }
    })
}
