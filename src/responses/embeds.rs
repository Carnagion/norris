use poise::serenity_prelude as serenity;

use serenity::{
    colours::{
        branding::BLURPLE,
        css::{DANGER, POSITIVE},
    },
    *,
};

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
                "You entered the name **{}**. \
                Is that correct?",
                name,
            ))
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
    }
}

pub fn registered_continue_embed(
    chat_channel_id: ChannelId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(POSITIVE)
            .description(format!(
                "Thank you for your patience! \
                Your registration is now **complete**. \
                You can head over to <#{}> to chat with your new course peers and mentors.
                However, before you go, we still have a few questions for you regarding your pronouns and accommodation. \
                Please note that these are completely optional and you can ignore or skip them if you wish to.",
                chat_channel_id,
            ))
    }
}

pub fn pronouns_embed() -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    |embed| {
        embed.title("Registration").colour(BLURPLE).description(
            "What are your **pronouns**? \
            This helps others understand how best to address you, \
            and will only be displayed via a role on your server profile.",
        )
    }
}

pub fn housing_embed() -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    |embed| {
        embed.title("Registration").colour(BLURPLE).description(
            "What kind of **accommodation** will you be staying in? \
            This helps you find others staying in the same accommodation or similar types, \
            and will only be displayed via a role on your server profile.",
        )
    }
}

pub fn registration_finished(
    chat_channel_id: ChannelId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(POSITIVE)
            .description(format!(
                "Thank you for your patience! \
                We have no additional questions. \
                If you haven't already done so, \
                you can head over to <#{}> to chat with your new course peers and mentors.",
                chat_channel_id,
            ))
    }
}

pub fn user_joined_log_embed(user_id: UserId) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!("<@{}> has joined the server.", user_id))
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
    }
}

pub fn registered_log_emed(
    user_id: UserId,
    kind: VerifiedUserKind,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(POSITIVE)
            .description(format!(
                "<@{}>, a **{}**, has completed registration.",
                user_id,
                kind.to_string()
            ))
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
    }
}
