pub mod commands;
pub mod events;

pub struct Data {}
pub type Context<'a> = poise::Context<'a, Data, crate::Error>;
