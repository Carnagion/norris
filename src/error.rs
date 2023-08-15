use chrono::Utc;

use poise::serenity_prelude as serenity;

use serenity::{colours::css::DANGER, Error as SerenityError, *};

use sqlx::Error as SqlError;

use thiserror::Error;

use crate::prelude::*;

/// Possible errors that can occur during [`Norris`]'s operation.
#[derive(Debug, Error)]
pub enum BotError {
    /// A Discord-related error, either returned by custom Discord handler code or by the Discord API.
    #[error("discord error: {}", .0)]
    Discord(#[from] SerenityError),
    /// A database-related error.
    #[error("{}", .0)]
    Sql(#[from] SqlError),
}

#[tracing::instrument(skip_all)]
pub(crate) async fn report_framework_error(error: BotFrameworkError<'_>) {
    let result = match error {
        BotFrameworkError::Command { error, ctx } => {
            tracing::error!("Failed to execute command: {}", error);
            embed(&ctx, |embed| {
                embed
                    .title("Error")
                    .colour(DANGER)
                    .description(format!("Something went wrong ({}).", error))
                    .timestamp(Utc::now())
            })
            .await
        },
        BotFrameworkError::ArgumentParse { error, input, ctx } => {
            tracing::error!("Failed to parse command arguments: {}", error);
            embed(&ctx, |embed| {
                embed
                    .title("Error")
                    .colour(DANGER)
                    .description("You provided one or more invalid arguments.");
                if let Some(input) = input {
                    embed.field(
                        "Argument",
                        format!(
                            "```\n{}\n```",
                            input.replace("```", "\u{200B}`\u{200B}`\u{200B}`"), // NOTE: Zero width spaces
                        ),
                        false,
                    );
                }
                embed.timestamp(Utc::now())
            })
            .await
        },
        BotFrameworkError::CommandStructureMismatch { description, ctx } => {
            tracing::error!("Unexpected command structure: {}", description);
            embed(&BotContext::Application(ctx), |embed| {
                embed
                    .title("Error")
                    .colour(DANGER)
                    .description(format!("Invalid command structure ({}).", description))
                    .timestamp(Utc::now())
            })
            .await
        },
        BotFrameworkError::CommandCheckFailed { error, ctx } => {
            if let Some(error) = error {
                tracing::error!("Command verification check failed: {}", error);
            }
            embed(&ctx, |embed| {
                embed
                    .title("Error")
                    .colour(DANGER)
                    .description("One of the command verification checks failed.")
                    .timestamp(Utc::now())
            })
            .await
        },
        BotFrameworkError::CooldownHit {
            remaining_cooldown,
            ctx,
        } => {
            embed(&ctx, |embed| {
                embed
                    .title("Error")
                    .colour(DANGER)
                    .description("That command is under cooldown.")
                    .field(
                        "Time remaining",
                        format!("{} second(s)", remaining_cooldown.as_secs()),
                        false,
                    )
                    .timestamp(Utc::now())
            })
            .await
        },
        BotFrameworkError::MissingBotPermissions {
            missing_permissions,
            ctx,
        } => {
            tracing::error!("Bot requires permissions: {}", missing_permissions);
            embed(&ctx, |embed| {
                embed
                    .title("Error")
                    .colour(DANGER)
                    .description("I require more permissions to execute that command.")
                    .field(
                        "Missing persmissions",
                        format!("```\n{}\n```", missing_permissions),
                        false,
                    )
                    .timestamp(Utc::now())
            })
            .await
        },
        BotFrameworkError::MissingUserPermissions {
            missing_permissions,
            ctx,
        } => {
            embed(&ctx, |embed| {
                embed
                    .title("Error")
                    .colour(DANGER)
                    .description("You require more permissions to use that command.");
                if let Some(perms) = missing_permissions {
                    embed.field("Missing permissions", format!("```\n{}\n```", perms), false);
                }
                embed.timestamp(Utc::now())
            })
            .await
        },
        BotFrameworkError::NotAnOwner { ctx } => {
            embed(&ctx, |embed| {
                embed
                    .title("Error")
                    .colour(DANGER)
                    .description("Only the bot developers can use that command.")
                    .timestamp(Utc::now())
            })
            .await
        },
        BotFrameworkError::GuildOnly { ctx } => {
            embed(&ctx, |embed| {
                embed
                    .title("Error")
                    .colour(DANGER)
                    .description("You can only use that command in a guild.")
                    .timestamp(Utc::now())
            })
            .await
        },
        BotFrameworkError::DmOnly { ctx } => {
            embed(&ctx, |embed| {
                embed
                    .title("Error")
                    .colour(DANGER)
                    .description("You can only use that command in a direct message.")
                    .timestamp(Utc::now())
            })
            .await
        },
        BotFrameworkError::NsfwOnly { ctx } => {
            embed(&ctx, |embed| {
                embed
                    .title("Error")
                    .colour(DANGER)
                    .description("You can only use that command in a NSFW channel.")
                    .timestamp(Utc::now())
            })
            .await
        },
        BotFrameworkError::Setup { error, .. } => {
            tracing::error!("User data setup failed: {}", error);
            Ok(())
        },
        BotFrameworkError::EventHandler { error, event, .. } => {
            tracing::error!("Event handler for {} failed: {}", event.name(), error);
            Ok(())
        },
        BotFrameworkError::DynamicPrefix { error, msg, .. } => {
            tracing::error!(r#"Dynamic prefix failed on "{}": {}"#, msg.content, error);
            Ok(())
        },
        BotFrameworkError::UnknownCommand {
            prefix,
            msg_content,
            ..
        } => {
            tracing::error!(
                r#"Unrecognised command "{}" for prefix "{}""#,
                msg_content,
                prefix
            );
            Ok(())
        },
        BotFrameworkError::UnknownInteraction { .. } => {
            tracing::error!("Unknown interaction");
            Ok(())
        },
        non_exhaustive => {
            tracing::error!("Unknown error: {}", non_exhaustive);
            Ok(())
        },
    };

    if let Err(nested_error) = result {
        tracing::error!("Failed to report error: {}", nested_error);
    }
}

async fn embed(
    context: &BotContext<'_>,
    embed_builder: impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed,
) -> Result<()> {
    context
        .send(|response| response.embed(embed_builder).ephemeral(true))
        .await?;
    Ok(())
}
