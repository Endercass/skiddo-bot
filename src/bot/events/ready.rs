use poise::serenity_prelude::{self as serenity, Ready};
use scorched::{logf, LogData, LogImportance};

pub async fn ready_event(bot_info: &Ready, _: &serenity::Context) {
    logf!(
        Info,
        "{} has started and connected to discord.",
        bot_info.user.name
    );
}
