use teloxide::prelude::*;

use super::{favourite_music::FavouriteMusic, states::*};
use super::*;

pub type In<State> = TransitionIn<State, std::convert::Infallible>;
pub type Out = TransitionOut<Wrapper>;


pub async fn idle(cx: In<IdleState>) -> Out {
    let (cx, dialogue) = cx.unpack();

    if let Some(text) = cx.update.text() {
        let reply = if let Some(cmd) = utils::extract_bot_command(&text) {
            match cmd.as_str() {
                "newbot" => {
                    return new_bot_start(In { cx, dialogue: Ok(NewBotStartState) }).await;
                }
                _ => {
                    cx.answer_str("haha").await?;
                }
            }
        } else {
            cx.answer_str("hehe").await?;
        };

    }
    exit()
}


pub async fn start(cx: In<StartState>) -> Out {
    let (cx, dialogue) = cx.unpack();


    // cx.answer_str("输入正确命令").await?;
    // exit()

    cx.answer_str("Let's start! First, what's your full name?").await?;
    next(dialogue.up())
}

pub async fn receive_full_name(cx: In<ReceiveFullNameState>) -> Out {
    let (cx, dialogue) = cx.unpack();

    match cx.update.text_owned() {
        Some(full_name) => {
            cx.answer_str("What a wonderful name! Your age?").await?;
            next(dialogue.up(full_name))
        }
        _ => {
            cx.answer_str("Please, enter a text message!").await?;
            next(dialogue)
        }
    }
}

pub async fn receive_age(cx: In<ReceiveAgeState>) -> Out {
    let (cx, dialogue) = cx.unpack();

    match cx.update.text().map(str::parse) {
        Some(Ok(age)) => {
            cx.answer("Good. Now choose your favourite music:")
                .reply_markup(FavouriteMusic::markup())
                .send()
                .await?;
            next(dialogue.up(age))
        }
        _ => {
            cx.answer_str("Please, enter a number!").await?;
            next(dialogue)
        }
    }
}

pub async fn receive_favourite_music(
    cx: In<ReceiveFavouriteMusicState>,
) -> Out {
    let (cx, dialogue) = cx.unpack();

    match cx.update.text().map(str::parse) {
        Some(Ok(favourite_music)) => {
            cx.answer_str(format!("Fine. {}", dialogue.up(favourite_music)))
                .await?;
            exit()
        }
        _ => {
            cx.answer_str("Please, enter from the keyboard!").await?;
            next(dialogue)
        }
    }
}


pub async fn new_bot_start(cx: In<NewBotStartState>) -> Out {
    let (cx, dialogue) = cx.unpack();
    cx.answer_str("开始创建机器人! 首先, 给机器人起个名字").await?;
    return next(dialogue.up());
}

pub async fn receive_bot_name(cx: In<ReceiveBotNameState>) -> Out {
    let (cx, dialogue) = cx.unpack();


    match cx.update.text().map(str::parse) {
        Some(Ok(bot_name)) => {
            cx.answer_str(format!("很好，已经完成！\n{}", dialogue.up(bot_name))).await?;
            exit()
        }
        _ => {
            cx.answer_str("Please, enter a number!").await?;
            next(dialogue)
        }
    }
}