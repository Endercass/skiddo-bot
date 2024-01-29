pub mod commands;
pub mod events;
pub mod proxy;

pub struct Data {
    pub zbus_conn: zbus::Connection,
}
pub type Context<'a> = poise::Context<'a, Data, crate::Error>;
