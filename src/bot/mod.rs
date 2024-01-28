pub mod commands;

pub struct Data {}
pub type Context<'a> = poise::Context<'a, Data, crate::Error>;
