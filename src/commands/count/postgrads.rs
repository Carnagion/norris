use crate::prelude::*;

/// Count the number of registered postgraduate students.
#[poise::command(slash_command, guild_only)]
pub async fn postgrads(context: BotContext<'_>) -> BotResult<()> {
    // Defer reply to allow time for counting students
    context.defer().await?;

    // Count the total and registered postgrads
    let count = sqlx::query!(
        "select count(registered_user_id) as registered, count(*) as total
        from users
        where kind = ?",
        VerifiedUserKind::Postgrad.to_string(),
    )
    .fetch_one(&context.data().database_pool)
    .await?;

    // Reply to the command
    context
        .send(|reply| {
            reply.embed(embeds::registration::count(
                "Postgraduates",
                count.registered as u64,
                count.total as u64,
            ))
        })
        .await?;

    Ok(())
}
