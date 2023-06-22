use poise::serenity_prelude::{self as serenity, colours::css::DANGER};

use serenity::{colours::branding::BLURPLE, *};

use crate::prelude::*;

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
    }
}

pub fn request_name_embed() -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    |embed| {
        embed.title("Registration").colour(BLURPLE).description(
            "Please enter your **name** exactly as when you applied to the University.",
        )
    }
}

pub fn confirm_name_embed(name: &str) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + '_ {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!(
                "You entered the name `{}`. \
                Is that correct?",
                name
            ))
    }
}

pub fn confirm_kind_embed(
    kind: VerifiedUserKind,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .color(BLURPLE)
            .description(format!(
                "We've detected that you are a **{}**. \
                Is that correct?",
                kind.description(),
            ))
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
                "Please seek assistance in <#{}>.",
                support_channel_id,
            ))
    }
}
