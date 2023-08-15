use poise::ChoiceParameter;

use crate::prelude::*;

/// Add an unregistered user to the database.
#[poise::command(slash_command, guild_only, required_permissions = "ADMINISTRATOR")]
#[tracing::instrument(skip(context), err(Debug))]
pub async fn add(
    context: BotContext<'_>,
    #[description = "The user's name."] name: String,
    #[description = "What kind of user to add them as."] kind: StudentKind,
) -> BotResult<()> {
    // Defer reply to give time for database queries
    context.defer().await?;

    let kind = VerifiedUserKind::from(kind);

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

/// Represents the kinds of users that can be added as unregistered users via the `registration add` command.
///
/// For security reasons, only undergraduates and postgraduates can be added via the command.
/// Mentors, senior mentors, honorary mentors, and members of faculty must be added manually, as they have significantly more permissions than regular students.
#[derive(ChoiceParameter, Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum StudentKind {
    /// An undergraduate student.
    #[default]
    #[name = "Undergraduate"]
    Undergrad,
    /// A postgraduate student.
    #[name = "Postgraduate"]
    Postgrad,
}

impl From<StudentKind> for VerifiedUserKind {
    fn from(kind: StudentKind) -> Self {
        match kind {
            StudentKind::Undergrad => Self::Undergrad,
            StudentKind::Postgrad => Self::Postgrad,
        }
    }
}
