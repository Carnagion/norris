use poise::serenity_prelude as serenity;

use serenity::*;

use crate::prelude::*;

pub async fn yes_clicked(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
    name: String,
) -> BotResult<()> {
    // Try to find a matching user
    let verified_user = sqlx::query!(
        "select * from users
        where name = ? and registered_user_id is null
        order by kind
        limit 1",
        name,
    )
    .try_map(|row| VerifiedUser::from_columns(row.name, row.kind, row.registered_user_id))
    .fetch_optional(&bot_data.database_pool)
    .await?;

    match verified_user {
        // No matching name was found
        None => {
            // Update the user's registration state to failed and ask them to seek assistance
            reset_status(
                context,
                component_interaction,
                bot_data,
                RegistrationStatus::Failed,
                component_interaction.user.id,
                embeds::registration::name_error(bot_data.channels.support_channel_id),
            )
            .await?;

            // Alert mentors about the name error
            bot_data
                .channels
                .log_channel_id
                .send_message(&context.http, |message| {
                    message
                        .content(format!(
                            "<@&{}> <@&{}>",
                            bot_data.roles.hierarchy.mentor_role_id,
                            bot_data.roles.hierarchy.admin_role_id,
                        ))
                        .embed(embeds::logs::name_error(
                            component_interaction.user.id,
                            &name,
                            bot_data.channels.support_channel_id,
                        ))
                })
                .await
        },
        // A matching name was found and ask the user to confirm their kind
        Some(verified_user) => {
            // Confirm the user's kind
            request_kind_confirm(context, component_interaction, bot_data, verified_user).await?;

            // Log the name confirmation
            bot_data
                .channels
                .log_channel_id
                .send_message(&context.http, |message| {
                    message.embed(embeds::logs::name_confirmed(
                        component_interaction.user.id,
                        &name,
                    ))
                })
                .await
        },
    }?;

    Ok(())
}

pub async fn no_clicked(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
) -> BotResult<()> {
    // Update the user's registration state to started and ask them to enter their name again
    reset_status(
        context,
        component_interaction,
        bot_data,
        RegistrationStatus::Started,
        component_interaction.user.id,
        embeds::registration::name_request(),
    )
    .await
}

async fn reset_status(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
    status: RegistrationStatus,
    user_id: UserId,
    embed: impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed,
) -> BotResult<()> {
    // Update the user's registration state
    sqlx::query!(
        "update registrations
        set status = ?, name = null
        where user_id = ?",
        status.to_string(),
        user_id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Display an embed
    component_interaction
        .create_followup_message(&context.http, |message| message.embed(embed))
        .await?;

    Ok(())
}

async fn request_kind_confirm(
    context: &Context,
    component_interaction: &MessageComponentInteraction,
    bot_data: &BotData,
    verified_user: VerifiedUser,
) -> BotResult<()> {
    // Update the user's registration state to name confirmed
    sqlx::query!(
        "update registrations
        set status = ?, kind = ?
        where user_id = ?", // NOTE: Name should have been set when name entered
        RegistrationStatus::KindFound(verified_user.name, verified_user.kind).to_string(),
        verified_user.kind.to_string(),
        component_interaction.user.id.0,
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Ask the user to confirm their kind
    component_interaction
        .create_followup_message(&context.http, |message| {
            message
                .embed(embeds::registration::kind_confirm(verified_user.kind))
                .components(components::confirm_kind_buttons())
        })
        .await?;

    Ok(())
}
