use chrono::Utc;

use poise::serenity_prelude::{self as serenity, colours::css::WARNING};

use serenity::{
    colours::{
        branding::BLURPLE,
        css::{DANGER, POSITIVE},
    },
    *,
};

use crate::prelude::*;

mod logs;
pub use logs::*;

pub fn instructions_embed(user_id: UserId) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!(
                "Welcome to the **University of Nottingham Computer Science** server, <@{}>! \
                We'll need a couple of details from you in order to get you set up.",
                user_id,
            ))
            .timestamp(Utc::now())
    }
}

pub fn instructions_sent_embed(
    user_id: UserId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!(
                "Welcome to the **University of Nottingham Computer Science** server, <@{}>! \
                Please check your direct messages for instructions on how to continue.",
                user_id,
            ))
            .timestamp(Utc::now())
    }
}

pub fn instructions_error_embed(
    user_id: UserId,
    support_channel_id: ChannelId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(DANGER)
            .description(format!(
                "Welcome to the **University of Nottingham Computer Science** server, <@{}>! \
                Unfortunately, there was an error in sending you instructions. \
                Please seek assistance in <#{}>.",
                user_id, support_channel_id,
            ))
            .timestamp(Utc::now())
    }
}

pub fn request_name_embed() -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(
                "Please enter your **name** exactly as when you applied to the University.",
            )
            .timestamp(Utc::now())
    }
}

pub fn confirm_name_embed(name: &str) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + '_ {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!(
                "You entered the name **{}**. \
                Is that correct?",
                name,
            ))
            .timestamp(Utc::now())
    }
}

pub fn no_name_error_embed(
    support_channel_id: ChannelId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(DANGER)
            .description(format!(
                "Unfortunately, we don't have that name in our system, or it has already been registered. \
                Please seek assistance in <#{}>.",
                support_channel_id,
            ))
            .timestamp(Utc::now())
    }
}

pub fn confirm_kind_embed(
    kind: VerifiedUserKind,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!(
                "We've detected that you are a **{}**. \
                Is that correct?",
                kind.description(),
            ))
            .timestamp(Utc::now())
    }
}

pub fn kind_error_embed(
    support_channel_id: ChannelId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(DANGER)
            .description(format!(
                "Unfortunately, our system might have assigned you incorrectly. \
                Please seek assistance in <#{}>.",
                support_channel_id,
            ))
            .timestamp(Utc::now())
    }
}

pub fn verified_continue_embed() -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(POSITIVE)
            .description(
                "Thank you for your patience! \
                You are now **verified**, but not fully registered yet.
                We still have a few questions for you regarding your pronouns and accommodation. \
                Please note that you can skip these if you wish to. \
                Your registration will be complete after you answer or skip the following questions.",
            )
            .timestamp(Utc::now())
    }
}

pub fn pronouns_embed() -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(
                "What are your **pronouns**? \
                This helps others understand how best to address you, \
                and will only be displayed via a role on your server profile.",
            )
            .timestamp(Utc::now())
    }
}

pub fn housing_embed() -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(
                "What kind of **accommodation** will you be staying in? \
                This helps you find others staying in the same accommodation or similar types, \
                and will only be displayed via a role on your server profile.",
            )
            .timestamp(Utc::now())
    }
}

pub fn registration_finished_embed(
    main_channel_id: ChannelId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(POSITIVE)
            .description(format!(
                "Thank you for your patience! \
                We have no additional questions. \
                You can now head over to <#{}> to chat with your new course peers and mentors.",
                main_channel_id,
            ))
            .timestamp(Utc::now())
    }
}

pub fn registration_welcome_embed(
    user_id: UserId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!("Welcome to the server, <@{}>!", user_id))
            .timestamp(Utc::now())
    }
}

pub fn registration_restart_embed(
    user_id: UserId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(WARNING)
            .description(format!("Re-started <@{}>'s registration.", user_id,))
            .timestamp(Utc::now())
    }
}

pub fn registration_nuke_embed() -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    |embed| {
        embed
            .title("Registration")
            .colour(WARNING)
            .description("Nuked registrations.")
            .timestamp(Utc::now())
    }
}

pub fn count_embed(
    title: &str,
    registered: u64,
    total: u64,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + '_ {
    // Calculate percentage of registered vs total
    let ratio = registered as f64 / total as f64;
    let percent = ratio * 100.0;

    move |embed| {
        embed
            .title(title)
            .colour(BLURPLE)
            .field("Total", total, true)
            .field("Registered", registered, true)
            .field("Percentage", format!("{:.2}", percent), false)
            .timestamp(Utc::now())
    }
}

pub fn nickname_acknowledge_embed() -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
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

pub fn nickname_request_embed<'a>(
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

pub fn nickname_approved_embed(
    nickname: &str,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + '_ {
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

pub fn nickname_denied_embed(
    nickname: &str,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + '_ {
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
