use poise::serenity_prelude as serenity;

use serenity::*;

pub const INSTRUCTIONS_CONTINUE: &str = "instructions-continue";

pub fn instructions_continue_button() -> impl FnOnce(&mut CreateComponents) -> &mut CreateComponents
{
    |comp| {
        comp.create_action_row(|row| {
            row.create_button(|button| {
                button
                    .label("Continue")
                    .custom_id(INSTRUCTIONS_CONTINUE)
                    .style(ButtonStyle::Primary)
            })
        })
    }
}

pub fn instructions_sent_button() -> impl FnOnce(&mut CreateComponents) -> &mut CreateComponents {
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

pub const NAME_CONFIRM_YES: &str = "name-confirm-yes";

pub const NAME_CONFIRM_NO: &str = "name-confirm-no";

pub fn confirm_name_buttons() -> impl FnOnce(&mut CreateComponents) -> &mut CreateComponents {
    |comp| {
        comp.create_action_row(|row| {
            row.create_button(|button| {
                button
                    .label("Yes")
                    .custom_id(NAME_CONFIRM_YES)
                    .style(ButtonStyle::Success)
            })
            .create_button(|button| {
                button
                    .label("No")
                    .custom_id(NAME_CONFIRM_NO)
                    .style(ButtonStyle::Danger)
            })
        })
    }
}
