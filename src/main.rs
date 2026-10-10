use teloxide::{requests::Requester, types::Message, Bot};

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("BOT STARTING...");

    let bot = Bot::from_env();

    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        bot.send_dice(msg.chat.id).await?;
        Ok(())
    })
    .await;
}
