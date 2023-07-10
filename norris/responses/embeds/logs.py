from datetime import datetime

from discord import Colour, Embed

from norris.model import VerifiedUserKind


def user_joined(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has joined the server.",
        timestamp=datetime.utcnow(),
    )


def dm_error(user_id: int, support_channel_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.red(),
        description=f"There was an error in sending <@{user_id}> a direct message. "
                    f"They have been directed to <#{support_channel_id}>.",
        timestamp=datetime.utcnow(),
    )


def registration_started(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has started registration.",
        timestamp=datetime.utcnow(),
    )


def name_confirmed(user_id: int, name: str) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has confirmed their name as **{name}**.",
        timestamp=datetime.utcnow(),
    )


def name_error(user_id: int,
               support_channel_id: int,
               name: str) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.red(),
        description=f"<@{user_id}> has entered the name **{name}**, but it appears to "
                    f"be invalid or already registered. They have been redirected to "
                    f"<#{support_channel_id}>.",
        timestamp=datetime.utcnow(),
    )


def kind_error(user_id: int,
               kind: VerifiedUserKind,
               support_channel_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.red(),
        description=f"<@{user_id}> has indicated that they have been incorrectly "
                    f"marked as a **{kind.description()}**. They have been redirected "
                    f"to <#{support_channel_id}>.",
        timestamp=datetime.utcnow(),
    )


def verified(user_id: int, kind: VerifiedUserKind) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.green(),
        description=f"<@{user_id}> has been **verified** as a **"
                    f"{kind.description()}**.",
        timestamp=datetime.utcnow(),
    )


def pronouns_picked(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has selected pronouns.",
        timestamp=datetime.utcnow(),
    )


def housing_picked(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has selected housing.",
        timestamp=datetime.utcnow(),
    )


def registration_finished(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.green(),
        description=f"<@{user_id}> has **completed** registration.",
        timestamp=datetime.utcnow(),
    )


def user_left(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour(0xFAA81A),  # NOTE: official Discord warning colour
        description=f"<@{user_id}> has left the server. They have been de-registered.",
        timestamp=datetime.utcnow(),
    )


def registration_restarted(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour(0xFAA81A),  # NOTE: official Discord warning colour
        description=f"<@{user_id}> has **re-started** registration.",
        timestamp=datetime.utcnow(),
    )
