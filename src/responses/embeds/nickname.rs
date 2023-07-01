use chrono::Utc;

use poise::serenity_prelude as serenity;

use serenity::{
    colours::{
        branding::BLURPLE,
        css::{DANGER, POSITIVE},
    },
    *,
};

pub fn acknowledge_request() -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    |embed| {
        embed
            .title("Nickname request")
            .colour(BLURPLE)
            .description(
                "Your nickname request has been received. \
                Please wait for approval by a mentor.",
            )
            .timestamp(Utc::now())
    }
}

pub fn request_approval<'a>(
    user_id: UserId,
    name: &'a str,
    current_nickname: &'a str,
    requested_nickname: &'a str,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + 'a {
    move |embed| {
        embed
            .title("Nickname request")
            .colour(BLURPLE)
            .description(format!("<@{}> has requested a new nickname.", user_id))
            .field("Current nickname", current_nickname, true)
            .field("Requested nickname", requested_nickname, true)
            .field("Verified name", name, false)
            .timestamp(Utc::now())
    }
}

pub fn approved(nickname: &str) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + '_ {
    move |embed| {
        embed
            .title("Nickname request")
            .colour(POSITIVE)
            .description(format!(
                "Your request to change your nickname to **{}** was **approved**.",
                nickname,
            ))
            .timestamp(Utc::now())
    }
}

pub fn denied(nickname: &str) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + '_ {
    move |embed| {
        embed
            .title("Nickname request")
            .colour(DANGER)
            .description(format!(
                "Your request to change your nickname to **{}** was **denied**. \
                Please note that you can message a mentor if you think this was a mistake.",
                nickname,
            ))
            .timestamp(Utc::now())
    }
}
