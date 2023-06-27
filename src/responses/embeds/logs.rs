use chrono::Utc;

use poise::serenity_prelude as serenity;

use serenity::{
    colours::{
        branding::BLURPLE,
        css::{DANGER, POSITIVE, WARNING},
    },
    *,
};

use crate::prelude::*;

pub fn user_joined_log_embed(user_id: UserId) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!("<@{}> has joined the server.", user_id))
            .timestamp(Utc::now())
    }
}

pub fn dm_error_log_embed(
    user_id: UserId,
    support_channel_id: ChannelId,
    mentor_role_id: RoleId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(DANGER)
            .description(format!(
                "There was an error in sending <@{}> a direct message. \
                They have been redirected to <#{}>, and a <@&{}>'s assistance is required.",
                user_id, support_channel_id, mentor_role_id,
            ))
            .timestamp(Utc::now())
    }
}

pub fn registration_started_log_embed(
    user_id: UserId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!("<@{}> has started registration.", user_id))
            .timestamp(Utc::now())
    }
}

pub fn name_confirmed_log_embed(
    user_id: UserId,
    name: &str,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + '_ {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!(
                "<@{}> has confirmed their name as **{}**.",
                user_id, name,
            ))
            .timestamp(Utc::now())
    }
}

pub fn no_name_log_embed(
    user_id: UserId,
    name: &str,
    support_channel_id: ChannelId,
    mentor_role_id: RoleId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + '_ {
    move |embed| {
        embed
            .title("Registration")
            .colour(DANGER)
            .description(format!(
                "<@{}> has entered the name **{}**, but no such name was found, or it has already been registered. \
                They have been redirected to <#{}>, and a <@&{}>'s assistance is required.",
                user_id, name, support_channel_id, mentor_role_id,
            ))
            .timestamp(Utc::now())
    }
}

pub fn kind_error_log_embed(
    user_id: UserId,
    kind: VerifiedUserKind,
    support_channel_id: ChannelId,
    mentor_role_id: RoleId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(DANGER)
            .description(format!(
                "<@{}> has indicated that they have been incorrectly identified as a **{}**. \
                They have been redirected to <#{}>, and a <@&{}>'s assistance is required.",
                user_id,
                kind.description(),
                support_channel_id,
                mentor_role_id,
            ))
            .timestamp(Utc::now())
    }
}

pub fn verified_log_embed(
    user_id: UserId,
    kind: VerifiedUserKind,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(POSITIVE)
            .description(format!(
                "<@{}>, a **{}**, has been verified.",
                user_id,
                kind.description(),
            ))
            .timestamp(Utc::now())
    }
}

pub fn registered_log_embed(user_id: UserId) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(POSITIVE)
            .description(format!("<@{}> has **completed** registration.", user_id))
            .timestamp(Utc::now())
    }
}

pub fn user_left_log_embed(user_id: UserId) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!(
                "<@{}> has left the server. \
                They have been de-registered.",
                user_id,
            ))
            .timestamp(Utc::now())
    }
}

pub fn registration_restart_log_embed(
    user_id: UserId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(WARNING)
            .description(format!("<@{}> has re-started their registration.", user_id))
            .timestamp(Utc::now())
    }
}
