//! Embeds used in messages related to the registration process.

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

/// Embed builder for displaying registration instructions to a [`User`].
pub fn instructions(user_id: UserId) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
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

/// Embed builder for notifying a [`User`] to check their direct messages for instructions.
pub fn instructions_sent(user_id: UserId) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
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

/// Embed builder for notifying a [`User`] of errors while sending registration instructions.
pub fn instructions_error(
    user_id: UserId,
    support_channel_id: ChannelId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(DANGER)
            .description(format!(
                "Welcome to the **University of Nottingham Computer Science** server, <@{}>! \
                 Unfortunately, there was an error in sending you instructions. Please seek \
                 assistance in <#{}>.",
                user_id, support_channel_id,
            ))
            .timestamp(Utc::now())
    }
}

/// Embed builder for requesting a [`User`] to enter their name.
pub fn name_request() -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(
                "Please enter your **first and last name** exactly as when you applied to the \
                 University, in the format `firstname lastname`.",
            )
            .timestamp(Utc::now())
    }
}

/// Embed builder for requesting a [`User`] to confirm their name.
pub fn name_confirm(name: &str) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + '_ {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!(
                "You entered the name **{}**. Is that correct?",
                name,
            ))
            .timestamp(Utc::now())
    }
}

/// Embed builder for when a name is not in the database or is already registered.
pub fn name_error(
    support_channel_id: ChannelId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(DANGER)
            .description(format!(
                "Unfortunately, we don't have that name in our system, or it has already been \
                 registered. Please seek assistance in <#{}>.",
                support_channel_id,
            ))
            // TODO: Remove once we have a list of names
            .field(
                "Note",
                format!(
                    "As we do not have an official list of names yet, we are temporarily \
                     registering students manually. When seeking assistance in <#{}>, please \
                     state whether you are an **undergraduate or postgraduate student**, and if you \
                     **already have a confirmed place at the University** either through your results \
                     or an unconditional offer.",
                    support_channel_id,
                ),
                false,
            )
            .timestamp(Utc::now())
    }
}

/// Embed builder for requesting a [`User`] to confirm their [`VerifiedUserKind`].
pub fn kind_confirm(kind: VerifiedUserKind) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!(
                "We've detected that you are a **{}**. Is that correct?",
                kind.description(),
            ))
            .timestamp(Utc::now())
    }
}

/// Embed builder for when a [`VerifiedUserKind`] is detected incorrectly.
pub fn kind_error(
    support_channel_id: ChannelId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(DANGER)
            .description(format!(
                "Unfortunately, our system might have assigned you incorrectly. Please seek \
                 assistance in <#{}>.",
                support_channel_id,
            ))
            // TODO: Remove once we have a list of names
            .field(
                "Note",
                format!(
                    "As we do not have an official list of names yet, we are temporarily \
                     registering students manually. When seeking assistance in <#{}>, please \
                     state whether you are an undergraduate or postgraduate student, and if you \
                     already have a confirmed place at the University either through your results \
                     or an unconditional offer.",
                    support_channel_id,
                ),
                false,
            )
            .timestamp(Utc::now())
    }
}

/// Embed builder for when a [`User`] is verified.
pub fn verified() -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(POSITIVE)
            .description(
                "Thank you for your patience! You are now **verified**, but not fully registered \
                 yet.
                 We still have a few questions for you regarding your pronouns and accommodation. \
                 Please note that you can skip these if you wish to. Your registration will be \
                 complete after you answer or skip the following questions.",
            )
            .timestamp(Utc::now())
    }
}

/// Embed builder for asking a [`User`] their pronouns.
pub fn pronouns() -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(
                "What are your **pronouns**? Displaying your pronouns is important for making \
                 everyone on the server feel comfortable - they are for everyone regardless of if \
                 you are cis or under the trans umbrella. If you use multiple pronouns, or want \
                 to change your pronouns later, you can change them at any time in the pronouns \
                 channel once you are registered.",
            )
            .timestamp(Utc::now())
    }
}

/// Embed builder for asking a [`User`] their accommodation.
pub fn housing() -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(
                "What kind of **accommodation** will you be staying in? This helps you find \
                 others staying in the same accommodation or similar types, and will only be \
                 displayed via a role on your server profile.",
            )
            .timestamp(Utc::now())
    }
}

/// Embed builder for indicating registration completion.
pub fn finished(main_channel_id: ChannelId) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(POSITIVE)
            .description(format!(
                "Thank you for your patience! Your registration is now **complete**. We have no \
                 additional questions. You can now head over to <#{}> to chat with your new \
                 course peers and mentors.",
                main_channel_id,
            ))
            .timestamp(Utc::now())
    }
}

/// Embed builder for welcoming a [`User`] after their registration is complete.
pub fn welcome(user_id: UserId) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!("Welcome to the server, <@{}>!", user_id))
            .timestamp(Utc::now())
    }
}

/// Embed builder for indicating the restart of a [`User`]'s registration.
pub fn restart(user_id: UserId) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(WARNING)
            .description(format!("Re-started <@{}>'s registration.", user_id,))
            .timestamp(Utc::now())
    }
}

/// Embed builder for indicating that a registration nuke occurred.
pub fn nuke() -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    |embed| {
        embed
            .title("Registration")
            .colour(WARNING)
            .description("Nuked registrations.")
            .timestamp(Utc::now())
    }
}

/// Embed builder for indicating a manual addition to the users database.
pub fn add(
    name: &str,
    kind: VerifiedUserKind,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed + '_ {
    move |embed| {
        embed
            .title("Registration")
            .colour(WARNING)
            .description(format!(
                "Added **{}**, a **{}**, as an unregistered user.",
                name,
                kind.description(),
            ))
            .timestamp(Utc::now())
    }
}

/// Embed builder for displaying the count of registered students.
pub fn count(
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
