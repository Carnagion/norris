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

pub const PRONOUNS_HE_HIM: &str = "pronouns-he-him";

pub const PRONOUNS_SHE_HER: &str = "pronouns-she-her";

pub const PRONOUNS_THEY_THEM: &str = "pronouns-they-them";

pub const PRONOUNS_XE_XEM: &str = "pronouns-xe-xem";

pub const PRONOUNS_ANY: &str = "pronouns-any";

pub const PRONOUNS_ASK: &str = "pronouns-ask";

pub const PRONONS_SKIP: &str = "pronouns-skip";

pub fn pronouns_buttons() -> impl FnOnce(&mut CreateComponents) -> &mut CreateComponents {
    |comp| {
        comp.create_action_row(|row| {
            row.create_button(|button| {
                button
                    .label("He / Him")
                    .custom_id(PRONOUNS_HE_HIM)
                    .style(ButtonStyle::Primary)
            })
            .create_button(|button| {
                button
                    .label("She / Her")
                    .custom_id(PRONOUNS_SHE_HER)
                    .style(ButtonStyle::Primary)
            })
            .create_button(|button| {
                button
                    .label("They / Them")
                    .custom_id(PRONOUNS_THEY_THEM)
                    .style(ButtonStyle::Primary)
            })
            .create_button(|button| {
                button
                    .label("Xe / Xem")
                    .custom_id(PRONOUNS_XE_XEM)
                    .style(ButtonStyle::Primary)
            })
            .create_button(|button| {
                button
                    .label("Any")
                    .custom_id(PRONOUNS_ANY)
                    .style(ButtonStyle::Primary)
            })
        })
        // NOTE: An action row can only hold up to five buttons
        .create_action_row(|row| {
            row.create_button(|button| {
                button
                    .label("Ask me")
                    .custom_id(PRONOUNS_ASK)
                    .style(ButtonStyle::Secondary)
            })
            .create_button(|button| {
                button
                    .label("Skip")
                    .custom_id(PRONONS_SKIP)
                    .style(ButtonStyle::Danger)
            })
        })
    }
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
