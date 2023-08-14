use crate::prelude::*;

/// Add an unregistered user to the database.
#[poise::command(slash_command, guild_only, required_permissions = "ADMINISTRATOR")]
pub async fn add(
    context: BotContext<'_>,
    #[description = "The user's name."] name: String,
    #[description = "What kind of user to add them as."] kind: VerifiedUserKind,
) -> BotResult<()> {
    // Defer reply to give time for database queries
    context.defer().await?;

    // Add the user to the database
    sqlx::query!(
        "insert into users
        value (0, ?, ?, null)",
        name,
        kind.to_string(),
    )
    .execute(&context.data().database_pool)
    .await?;

    // Reply after adding
    context
        .send(|reply| reply.embed(embeds::registration::add(&name, kind)))
        .await?;

    Ok(())
}
