use poise::serenity_prelude::{
    colours::branding::BLURPLE, ButtonStyle, ChannelId, Context, CreateComponents, CreateEmbed,
    Member, UserId,
};

use crate::{model::RegistrationStatus, BotData, BotError};

pub async fn setup_registration(
    context: &Context,
    member: &Member,
    bot_data: &BotData,
) -> Result<(), BotError> {
    // Add the member as unregistered to the table of registration info
    sqlx::query!(
        "insert into registrations value (?, ?, null)",
        member.user.id.0,
        RegistrationStatus::Unregistered.to_string(),
    )
    .execute(&bot_data.database_pool)
    .await?;

    // Send instructions for registration to the user's DMs
    let instructions_sent = member
        .user
        .direct_message(&context.http, |message| {
            message
                .embed(instructions_dm_embed(member.user.id))
                .components(instructions_dm_button())
        })
        .await;
    match instructions_sent {
        Ok(_) => {
            bot_data
                .arrival_channel_id
                .send_message(&context.http, |message| {
                    message
                        .embed(instructions_foyer_embed(member.user.id))
                        .components(instructions_foyer_button())
                })
                .await
        },
        // Notify user in foyer if instructions could not be sent
        Err(error) => {
            log::error!("{}", error);
            bot_data
                .arrival_channel_id
                .send_message(&context.http, |message| {
                    message.embed(instructions_error_embed(
                        member.user.id,
                        bot_data.support_channel_id,
                    ))
                })
                .await
        },
    }?;

    Ok(())
}

fn instructions_dm_embed(user_id: UserId) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
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

fn instructions_dm_button() -> impl FnOnce(&mut CreateComponents) -> &mut CreateComponents {
    |comp| {
        comp.create_action_row(|row| {
            row.create_button(|button| button.label("Continue").style(ButtonStyle::Primary))
        })
    }
}

fn instructions_foyer_embed(user_id: UserId) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
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

fn instructions_foyer_button() -> impl FnOnce(&mut CreateComponents) -> &mut CreateComponents {
    |comp| {
        comp.create_action_row(|row| {
            row.create_button(|button| {
                button
                    .label("Open direct messages")
                    .url("https://discordapp.com/channels/@me")
                    .style(ButtonStyle::Link)
            })
        })
    }
}

fn instructions_error_embed(
    user_id: UserId,
    support_channel_id: ChannelId,
) -> impl FnOnce(&mut CreateEmbed) -> &mut CreateEmbed {
    move |embed| {
        embed
            .title("Registration")
            .colour(BLURPLE)
            .description(format!(
                "Welcome to the **University of Nottingham Computer Science** server, <@{}>! \
                Unfortunately, there was an error in sending you instructions. \
                Please seek assistance in <#{}>.",
                user_id, support_channel_id,
            ))
    }
}
