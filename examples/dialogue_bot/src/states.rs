use teloxide::prelude::*;

use super::favourite_music::FavouriteMusic;
use parse_display::Display;

pub struct IdleState;

pub struct StartState;

pub struct ReceiveFullNameState {
    rest: StartState,
}

pub struct ReceiveAgeState {
    rest: ReceiveFullNameState,
    full_name: String,
}

pub struct ReceiveFavouriteMusicState {
    rest: ReceiveAgeState,
    age: u8,
}

#[derive(Display)]
#[display(
"Your full name: {rest.rest.full_name}, your age: {rest.age}, your \
     favourite music: {favourite_music}"
)]
pub struct ExitState {
    rest: ReceiveFavouriteMusicState,
    favourite_music: FavouriteMusic,
}



pub struct NewBotStartState;

pub struct ReceiveBotNameState {
    pub rest: NewBotStartState,
}

#[derive(Display)]
#[display(
"你新创建的机器人是：{bot_name}"
)]
pub struct ExitNewBotState {
    pub rest: ReceiveBotNameState,
    pub bot_name: String,
}

up!(
    StartState -> ReceiveFullNameState,
    ReceiveFullNameState + [full_name: String] -> ReceiveAgeState,
    ReceiveAgeState + [age: u8] -> ReceiveFavouriteMusicState,
    ReceiveFavouriteMusicState + [favourite_music: FavouriteMusic] -> ExitState,

    NewBotStartState -> ReceiveBotNameState,
    ReceiveBotNameState + [bot_name : String] -> ExitNewBotState,
);

pub type Dialogue = Coprod!(
    IdleState,

    StartState,
    ReceiveFullNameState,
    ReceiveAgeState,
    ReceiveFavouriteMusicState,

    NewBotStartState,
    ReceiveBotNameState
);

wrap_dialogue!(
    Wrapper(Dialogue),
    default Self(Dialogue::inject(IdleState)),
);



