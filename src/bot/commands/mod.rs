#[cfg(debug_assertions)]
pub mod debug;

pub mod eval;

pub fn commands() -> Vec<poise::Command<crate::bot::Data, crate::Error>> {
    let mut commands = vec![eval::eval()];
    #[cfg(debug_assertions)]
    {
        commands.extend(debug::debug_commands());
    }
    commands
}
