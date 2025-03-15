//! Embeds used when logging the registration process of a user.

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

/// Embed builder for logging when a [`User`] joins the [`Guild`].
pub fn user_joined(user_id: UserId) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!("<@{}> has joined the server.", user_id))
            .timestamp(Utc::now())
    }
}

/// Embed builder for logging when a [`User`] could not be sent a direct message.
pub fn dm_error(
    user_id: UserId,
    username: &str,
    support_channel_id: ChannelId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + '_ {
    move |embed| {
        embed
            .title("Registration")
            .colour(DANGER)
            .description(format!(
                "There was an error in sending <@{}> ({}) a direct message. They have been \
                 redirected to <#{}>.",
                user_id, username, support_channel_id,
            ))
            .timestamp(Utc::now())
    }
}

/// Embed builder for logging when a [`User`] starts registration.
pub fn registration_started(user_id: UserId) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!("<@{}> has started registration.", user_id))
            .timestamp(Utc::now())
    }
}

/// Embed builder for logging when a [`User`] confirms their name.
pub fn name_confirmed(
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

/// Embed builder for logging when a name does not exist or is already registered.
pub fn name_error<'a>(
    user_id: UserId,
    username: &'a str,
    name: &'a str,
    support_channel_id: ChannelId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + 'a {
    move |embed| {
        embed
            .title("Registration")
            .colour(DANGER)
            .description(format!(
                "<@{}> ({}) has entered the name **{}**, but no such name was found, or it has \
                 already been registered. They have been redirected to <#{}>.",
                user_id, username, name, support_channel_id,
            ))
            .timestamp(Utc::now())
    }
}

/// Embed builder for logging when a [`VerifiedUserKind`] is detected incorrectly.
pub fn kind_error(
    user_id: UserId,
    username: &str,
    kind: VerifiedUserKind,
    support_channel_id: ChannelId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + '_ {
    move |embed| {
        embed
            .title("Registration")
            .colour(DANGER)
            .description(format!(
                "<@{}> ({}) has indicated that they have been incorrectly identified as a **{}**. \
                 They have been redirected to <#{}>.",
                user_id,
                username,
                kind.description(),
                support_channel_id,
            ))
            .timestamp(Utc::now())
    }
}

/// Embed builder for logging when a [`User`] is verified.
pub fn verified(
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

/// Embed builder for logging when a [`User`] picks or skips pronouns.
pub fn pronouns_picked(user_id: UserId) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!("<@{}> has selected pronouns.", user_id))
            .timestamp(Utc::now())
    }
}

/// Embed builder for logging when a [`User`] picks or skips housing.
pub fn housing_picked(user_id: UserId) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!("<@{}> has selected housing.", user_id))
            .timestamp(Utc::now())
    }
}

/// Embed builder for logging registration completion.
pub fn registration_finished(user_id: UserId) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(POSITIVE)
            .description(format!("<@{}> has **completed** registration.", user_id))
            .timestamp(Utc::now())
    }
}

/// Embed builder for logging when a [`User`] leaves and gets de-registered.
pub fn user_left(user_id: UserId) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(WARNING)
            .description(format!(
                "<@{}> has left the server. They have been de-registered.",
                user_id,
            ))
            .timestamp(Utc::now())
    }
}

/// Embed builder for logging when a [`User`] restarts their registration.
pub fn registration_restarted(
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

/// Embed builder for logging when the bot is shutting down due to a signal.
pub fn shutting_down() -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    |embed| {
        embed
            .title("Shutdown")
            .colour(DANGER)
            .description("Shutting down due to receiving a request to terminate.")
            .timestamp(Utc::now())
    }
}
