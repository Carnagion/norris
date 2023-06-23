use poise::serenity_prelude as serenity;

use serenity::*;

pub const INSTRUCTIONS_CONTINUE: &str = "instructions-continue";

pub fn instructions_continue_button() -> impl FnOnce(&mut CreateComponents) -> &mut CreateComponents
{
    button_continue(INSTRUCTIONS_CONTINUE)
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
    buttons_yes_no(NAME_CONFIRM_YES, NAME_CONFIRM_NO)
}

pub const KIND_CONFIRM_YES: &str = "kind-confirm-yes";

pub const KIND_CONFIRM_NO: &str = "kind-confirm-no";

pub fn confirm_kind_buttons() -> impl FnOnce(&mut CreateComponents) -> &mut CreateComponents {
    buttons_yes_no(KIND_CONFIRM_YES, KIND_CONFIRM_NO)
}

pub const OPTIONAL_CONTINUE: &str = "optional-continue";

pub fn registered_continue_button() -> impl FnOnce(&mut CreateComponents) -> &mut CreateComponents {
    button_continue(OPTIONAL_CONTINUE)
}

fn button_continue(
    id: &'static str,
) -> impl FnOnce(&mut CreateComponents) -> &mut CreateComponents {
    move |comp| {
        comp.create_action_row(|row| {
            row.create_button(|button| {
                button
                    .label("Continue")
                    .custom_id(id)
                    .style(ButtonStyle::Primary)
            })
        })
    }
}

fn buttons_yes_no(
    yes: &'static str,
    no: &'static str,
) -> impl FnOnce(&mut CreateComponents) -> &mut CreateComponents {
    move |comp| {
        comp.create_action_row(|row| {
            row.create_button(|button| {
                button
                    .label("Yes")
                    .custom_id(yes)
                    .style(ButtonStyle::Success)
            })
            .create_button(|button| button.label("No").custom_id(no).style(ButtonStyle::Danger))
        })
    }
}
