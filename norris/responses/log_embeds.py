from datetime import datetime

from discord import Colour, Embed

from ..model import VerifiedUserKind


def user_joined_log_embed(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has joined the server.",
        timestamp=datetime.utcnow(),
    )


def dm_error_log_embed(user_id: int,
                       mentor_role_id: int,
                       support_channel_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.red(),
        description=f"There was an error in sending <@{user_id}> a direct message. "
                    f"They have been directed to <#{support_channel_id}>, and a <@&"
                    f"{mentor_role_id}>'s assistance is required.",
        timestamp=datetime.utcnow(),
    )


def registration_started_log_embed(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has started registration.",
        timestamp=datetime.utcnow(),
    )


def name_confirmed_log_embed(user_id: int, name: str) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has confirmed their name as **{name}**.",
        timestamp=datetime.utcnow(),
    )


def name_error_log_embed(user_id: int,
                         mentor_role_id: int,
                         support_channel_id: int,
                         name: str) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.red(),
        description=f"<@{user_id}> has entered the name **{name}**, but it appears to "
                    f"be invalid or already registered. They have been redirected to "
                    f"<#{support_channel_id}>, and a <@&{mentor_role_id}>'s "
                    f"assistance is required.",
        timestamp=datetime.utcnow(),
    )


def kind_error_log_embed(user_id: int,
                         kind: VerifiedUserKind,
                         mentor_role_id: int,
                         support_channel_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.red(),
        description=f"<@{user_id}> has indicated that they have been incorrectly "
                    f"marked as a **{kind.description()}**. They have been redirected "
                    f"to <#{support_channel_id}>, and a <@&{mentor_role_id}>'s "
                    f"assistance is required.",
        timestamp=datetime.utcnow(),
    )


def verified_log_embed(user_id: int, kind: VerifiedUserKind) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.green(),
        description=f"<@{user_id}> has been **verified** as a **"
                    f"{kind.description()}**.",
        timestamp=datetime.utcnow(),
    )


def pronouns_selected_log_embed(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has selected pronouns.",
        timestamp=datetime.utcnow(),
    )


def housing_selected_log_embed(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has selected housing.",
        timestamp=datetime.utcnow(),
    )


def registered_log_embed(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.green(),
        description=f"<@{user_id}> has **completed** registration.",
        timestamp=datetime.utcnow(),
    )


def user_left_log_embed(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour(0xFAA81A),  # NOTE: official Discord warning colour
        description=f"<@{user_id}> has left the server. They have been de-registered.",
        timestamp=datetime.utcnow(),
    )


def registration_restarted_log_embed(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour(0xFAA81A),  # NOTE: official Discord warning colour
        description=f"<@{user_id}> has **re-started** registration.",
        timestamp=datetime.utcnow(),
    )
