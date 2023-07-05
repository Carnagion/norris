//! Message components used in the registration process and command replies.

use poise::serenity_prelude as serenity;

use serenity::*;

/// The ID of the `Continue` button for starting registration.
///
/// See [`instructions_continue_button`].
pub const INSTRUCTIONS_CONTINUE: &str = "instructions-continue";

/// A `Continue` button for starting registration.
///
/// See [`INSTRUCTIONS_CONTINUE`].
pub fn instructions_continue_button() -> impl FnOnce(&mut CreateComponents) -> &mut CreateComponents
{
    button_continue(INSTRUCTIONS_CONTINUE)
}

/// A link button that opens direct messages.
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

/// The ID of the `Yes` button when confirming a name.
///
/// See [`confirm_name_buttons`].
pub const NAME_CONFIRM_YES: &str = "name-confirm-yes";

/// The ID of the `No` button when confirming a name.
///
/// See [`confirm_name_buttons`].
pub const NAME_CONFIRM_NO: &str = "name-confirm-no";

/// `Yes` and `No` buttons for confirming a [`User`]'s name.
///
/// See [`NAME_CONFIRM_YES`] and [`NAME_CONFIRM_NO`].
pub fn confirm_name_buttons() -> impl FnOnce(&mut CreateComponents) -> &mut CreateComponents {
    buttons_yes_no("Yes", "No", NAME_CONFIRM_YES, NAME_CONFIRM_NO)
}

/// The ID of the `Yes` button when confirming a [`VerifiedUserKind`].
///
/// See [`confirm_kind_buttons`].
///
/// [`VerifiedUserKind`]: crate::prelude::VerifiedUserKind
pub const KIND_CONFIRM_YES: &str = "kind-confirm-yes";

/// The ID of the `No` button when confirming a [`VerifiedUserKind`].
///
/// [`VerifiedUserKind`]: crate::prelude::VerifiedUserKind
///
/// See [`confirm_kind_buttons`].
pub const KIND_CONFIRM_NO: &str = "kind-confirm-no";

/// `Yes` and `No` buttons for confirming a [`VerifiedUserKind`].
///
/// See [`KIND_CONFIRM_YES`] and [`KIND_CONFIRM_NO`].
///
/// [`VerifiedUserKind`]: crate::prelude::VerifiedUserKind
pub fn confirm_kind_buttons() -> impl FnOnce(&mut CreateComponents) -> &mut CreateComponents {
    buttons_yes_no("Yes", "No", KIND_CONFIRM_YES, KIND_CONFIRM_NO)
}

/// The ID of the `Continue` button after verification.
///
/// See [`verified_continue_button`].
pub const VERIFIED_CONTINUE: &str = "verified-continue";

/// A `Continue` button for proceeding past verification to the optional questions.
///
/// See [`VERIFIED_CONTINUE`].
pub fn verified_continue_button() -> impl FnOnce(&mut CreateComponents) -> &mut CreateComponents {
    button_continue(VERIFIED_CONTINUE)
}

/// The ID of the `He / Him` pronouns button.
///
/// See [`pronouns_buttons`].
pub const PRONOUNS_HE_HIM: &str = "pronouns-he-him";

/// The ID of the `She / Her` pronouns button.
///
/// See [`pronouns_buttons`].
pub const PRONOUNS_SHE_HER: &str = "pronouns-she-her";

/// The ID of the `They / Them` pronouns button.
///
/// See [`pronouns_buttons`].
pub const PRONOUNS_THEY_THEM: &str = "pronouns-they-them";

/// The ID of the `Xe / Xem` pronouns button.
///
/// See [`pronouns_buttons`].
pub const PRONOUNS_XE_XEM: &str = "pronouns-xe-xem";

/// The ID of the `Any` pronouns button.
///
/// See [`pronouns_buttons`].
pub const PRONOUNS_ANY: &str = "pronouns-any";

/// The ID of the `Ask me` pronouns button.
///
/// See [`pronouns_buttons`].
pub const PRONOUNS_ASK: &str = "pronouns-ask";

/// The ID of the `Skip` button for pronouns.
///
/// See [`pronouns_buttons`].
pub const PRONOUNS_SKIP: &str = "pronouns-skip";

/// Buttons for selecting (or skipping) pronouns.
///
/// See [`PRONOUNS_HE_HIM`], [`PRONOUNS_SHE_HER`], [`PRONOUNS_THEY_THEM`], [`PRONOUNS_XE_XEM`], [`PRONOUNS_ANY`], [`PRONOUNS_ASK`], and [`PRONOUNS_SKIP`].
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
                    .style(ButtonStyle::Primary)
            })
            .create_button(|button| {
                button
                    .label("Skip")
                    .custom_id(PRONOUNS_SKIP)
                    .style(ButtonStyle::Danger)
            })
        })
    }
}

/// The ID of the `Catered halls (Jubilee)` housing button.
///
/// See [`housing_buttons`].
pub const HOUSING_JC_CATERED: &str = "housing-jc-catered";

/// The ID of the `Self-atered halls (Jubilee area)` housing button.
///
/// See [`housing_buttons`].
pub const HOUSING_JC_SELF_CATERED: &str = "housing-jc-self-catered";

/// The ID of the `Catered halls (University Park)` housing button.
///
/// See [`housing_buttons`].
pub const HOUSING_UP_CATERED: &str = "housing-up-catered";

/// The ID of the `Self-atered halls (University Park area)` housing button.
///
/// See [`housing_buttons`].
pub const HOUSING_UP_SELF_CATERED: &str = "housing-up-self-catered";

/// The ID of the `Private housing` housing button.
///
/// See [`housing_buttons`].
pub const HOUSING_PRIVATE: &str = "housing-private";

/// The ID of the `Skip` button for housing.
///
/// See [`housing_buttons`].
pub const HOUSING_SKIP: &str = "housing-skip";

/// Buttons for selecting (or skipping) housing.
///
/// See [`HOUSING_JC_CATERED`], [`HOUSING_JC_SELF_CATERED`], [`HOUSING_UP_CATERED`], [`HOUSING_UP_SELF_CATERED`], [`HOUSING_PRIVATE`], and [`HOUSING_SKIP`].
pub fn housing_buttons() -> impl FnOnce(&mut CreateComponents) -> &mut CreateComponents {
    |comp| {
        comp.create_action_row(|row| {
            row.create_button(|button| {
                button
                    .label("Catered halls (Jubilee)")
                    .custom_id(HOUSING_JC_CATERED)
                    .style(ButtonStyle::Primary)
            })
            .create_button(|button| {
                button
                    .label("Catered halls (University Park)")
                    .custom_id(HOUSING_UP_CATERED)
                    .style(ButtonStyle::Primary)
            })
            .create_button(|button| {
                button
                    .label("Self-catered halls (Jubilee area)")
                    .custom_id(HOUSING_JC_SELF_CATERED)
                    .style(ButtonStyle::Primary)
            })
            .create_button(|button| {
                button
                    .label("Self-catered halls (University Park area)")
                    .custom_id(HOUSING_UP_SELF_CATERED)
                    .style(ButtonStyle::Primary)
            })
            .create_button(|button| {
                button
                    .label("Private housing")
                    .custom_id(HOUSING_PRIVATE)
                    .style(ButtonStyle::Primary)
            })
        })
        .create_action_row(|row| {
            row.create_button(|button| {
                button
                    .label("Skip")
                    .custom_id(HOUSING_SKIP)
                    .style(ButtonStyle::Danger)
            })
        })
    }
}

/// The ID of the `Approve` nickname button.
///
/// See [`nickname_approval_buttons`].
pub const NICKNAME_APPROVE: &str = "nickname-approve";

/// The ID of the `Deny` nickname button.
///
/// See [`nickname_approval_buttons`].
pub const NICKNAME_DENY: &str = "nickname-deny";

/// `Approve` and `Deny` buttons for nickname approval.
///
/// See [`NICKNAME_APPROVE`] and [`NICKNAME_DENY`].
pub fn nickname_approval_buttons() -> impl FnOnce(&mut CreateComponents) -> &mut CreateComponents {
    buttons_yes_no("Approve", "Deny", NICKNAME_APPROVE, NICKNAME_DENY)
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
    yes_text: &'static str,
    no_text: &'static str,
    yes_id: &'static str,
    no_id: &'static str,
) -> impl FnOnce(&mut CreateComponents) -> &mut CreateComponents {
    move |comp| {
        comp.create_action_row(|row| {
            row.create_button(|button| {
                button
                    .label(yes_text)
                    .custom_id(yes_id)
                    .style(ButtonStyle::Success)
            })
            .create_button(|button| {
                button
                    .label(no_text)
                    .custom_id(no_id)
                    .style(ButtonStyle::Danger)
            })
        })
    }
}
