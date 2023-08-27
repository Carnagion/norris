use crate::prelude::*;

/// Count the number of registered undergraduate students.
#[poise::command(slash_command, guild_only)]
#[tracing::instrument(skip_all, err(Debug))]
pub async fn undergrads(context: BotContext<'_>) -> BotResult<()> {
    // Defer reply to allow time for counting students
    context.defer().await?;

    // Count the total and registered undergrads
    let count = sqlx::query!(
        "select count(registered_user_id) as registered, count(*) as total
        from users
        where kind = ?",
        VerifiedUserKind::Undergrad.to_string(),
    )
    .fetch_one(&context.data().database_pool)
    .await?;

    // Reply to the command
    context
        .send(|reply| {
            reply.embed(embeds::registration::count(
                "Undergraduates",
                count.registered as u64,
                count.total as u64,
            ))
        })
        .await?;

    Ok(())
}
