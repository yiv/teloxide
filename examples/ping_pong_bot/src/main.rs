// This bot just answers "pong" to each incoming UpdateKind::Message.

use teloxide::prelude::*;

use std::env;
use teloxide::types::ParseMode;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting ping_pong_bot!");

    // env::set_var("TELOXIDE_TOKEN", "1475640165:AAGhKf8z9I91rYyadH2uVYU-eVqTpI9pIzo");
    env::set_var("TELOXIDE_TOKEN", "1014737762:AAGLklqzqkOFlO_ZlT5YRpJ2KnVOrkSvQ2E");


    let bot = Bot::from_env();
    println!("{:?}", bot);


    let me = bot.get_me().send().await.unwrap();
    println!("{:?}", me);

    Dispatcher::new(bot)
        .messages_handler(|rx: DispatcherHandlerRx<Message>| {
            rx.for_each(|message| async move {

                let s = r#"```python
pre-formatted fixed-width code block written in the Python programming language
```"#;
                message.answer(s).parse_mode(ParseMode::MarkdownV2).send().await;
                // message.answer_str("pong").await.log_on_error().await;
            })
        })
        .dispatch()
        .await;
}
