use teloxide::{prelude::*, utils::command::BotCommand};
use rand::{thread_rng, Rng};
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use std::env;

#[derive(BotCommand)]
#[command(rename = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "display this text.")]
    Help,
    #[command(description = "be a cat.")]
    Meow,
    #[command(description = "generate a random number within [0; 1).")]
    Generate,
}

fn generate() -> String {
    thread_rng().gen_range(0.0, 1.0).to_string()
}

async fn answer(
    cx: UpdateWithCx<Message>,
    command: Command,
) -> ResponseResult<()> {
    match command {
        Command::Help => {

            let mut markup = InlineKeyboardMarkup::default();
            markup = markup.append_to_row(InlineKeyboardButton::callback("TEST".to_string(), "2222".to_string()), 0);
            cx.answer(Command::descriptions()).reply_markup(markup).send().await? },
        Command::Generate => cx.answer(generate()).send().await?,
        Command::Meow => cx.answer("I am a cat! Meow!").send().await?,
    };

    Ok(())
}

async fn handle_commands(rx: DispatcherHandlerRx<Message>) {
    rx.commands::<Command, &str>("test_bot")
        .for_each_concurrent(None, |(cx, command, _)| async move {
            answer(cx, command).await.log_on_error().await;
        })
        .await;
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting simple_commands_bot!");

    env::set_var("TELEGRAM_API_URL", format!("https://api.telegram.org"));
    env::set_var("TELOXIDE_TOKEN", format!("1014737762:AAFikedCuhPKUaVq9ErA2PBiEtPkIFgDadU"));
    // env::set_var("TELOXIDE_TOKEN", format!("1475640165:AAGhKf8z9I91rYyadH2uVYU-eVqTpI9pIzo"));
    let bot = Bot::from_env();

    println!("{:?}", bot.get_me());
    Dispatcher::new(bot).messages_handler(handle_commands).dispatch().await;
}
