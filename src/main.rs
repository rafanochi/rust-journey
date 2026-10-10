use teloxide::{
    repls::CommandReplExt,
    requests::{Requester, ResponseResult},
    types::Message,
    utils::command::BotCommands,
    Bot,
};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("BOT STARTING...");

    let bot = Bot::from_env();

    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "Supported commands are:")]
enum Command {
    #[command(description = "Diplay help text")]
    Help,
    #[command(description = "Handle username")]
    Username(String),
    #[command(description = "handle a username and an age.", parse_with = "split")]
    NameAndage { name: String, age: u8 },
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => {
            bot.send_message(msg.chat.id, Command::descriptions().to_string())
                .await?
        }
        Command::Username(username) => {
            bot.send_message(msg.chat.id, format!("your username is @{username}"))
                .await?
        }
        Command::NameAndage { name, age } => {
            bot.send_message(msg.chat.id, format!("name: {name}, age: {age}"))
                .await?
        }
    };
    Ok(())
}
