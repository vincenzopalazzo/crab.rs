use teloxide::{prelude::*, utils::command::BotCommands};
mod cmd;

use crate::cmd::{answer, Command};

struct CrabBot;

impl CrabBot {
    async fn bind() {
        pretty_env_logger::init();
        log::info!("Starting command bot...");
        let bot = Bot::from_env().auto_send();
        teloxide::commands_repl(bot, answer, Command::ty()).await;
    }
}
