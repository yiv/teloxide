// This is a bot that asks your full name, your age, your favourite kind of
// music and sends all the gathered information back.
//
// # Example
// ```
//  - Let's start! First, what's your full name?
//  - Luke Skywalker
//  - What a wonderful name! Your age?
//  - 26
//  - Good. Now choose your favourite music
// *A keyboard of music kinds is displayed*
// *You select Metal*
//  - Metal
//  - Fine. Your full name: Luke Skywalker, your age: 26, your favourite music: Metal
// ```

#![allow(clippy::trivial_regex)]
#![allow(dead_code)]

#[macro_use]
extern crate teloxide;

use teloxide::prelude::*;
use teloxide::types::{InlineQuery, InlineQueryResult};

mod favourite_music;
mod states;
mod transitions;
mod utils;

use std::env;

use states::*;
use transitions::*;

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    env::set_var("TELEGRAM_API_URL", format!("https://api.telegram.org"));
    env::set_var("TELOXIDE_TOKEN", format!("1014737762:AAFikedCuhPKUaVq9ErA2PBiEtPkIFgDadU"));

    // env::set_var("TELEGRAM_API_URL", format!("http://129.204.249.76:30001/api/bot"));
    // env::set_var("TELOXIDE_TOKEN", format!("114995808754995200"));

    // teloxide::enable_logging!();
    log::info!("Starting dialogue_bot!");

    let bot = Bot::from_env();

    Dispatcher::new(bot)
        .messages_handler(
            // handle_dialogue
            DialogueDispatcher::new(|cx: DialogueWithCx<Message, Wrapper, std::convert::Infallible>| async move {
                let DialogueWithCx { cx, dialogue } = cx;

                // Unwrap without panic because of std::convert::Infallible.
                let Wrapper(dialogue) = dialogue.unwrap();


                dispatch!(
                    [cx, dialogue] ->
                    [idle, start, receive_full_name, receive_age, receive_favourite_music, new_bot_start, receive_bot_name]
                    ).expect("Something wrong with the bot!")
            })
        )
        .dispatch()
        .await;
}


