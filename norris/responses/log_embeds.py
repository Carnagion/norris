from datetime import datetime

from discord import Colour, Embed

from ..model import VerifiedUserKind

def user_join_log_embed(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has joined the server.",
        timestamp=datetime.utcnow(),
    )

def reg_started_log_embed(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has begun the registration process.",
        timestamp=datetime.utcnow(),
    )

def name_entered_log_embed(user_id: int, name: str) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has confirmed their name as **{name}**.",
        timestamp=datetime.utcnow(),
    )

def role_confirmed_log_embed(user_id: int, user_kind: VerifiedUserKind) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has been confirmed as a **{user_kind.description()}**.",
        timestamp=datetime.utcnow(),
    )

def verified_log_embed(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has been verified.",
        timestamp=datetime.utcnow(),
    )

def pronouns_selected_log_embed(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has added pronouns.",
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
        colour=Colour.blurple(),
        description=f"<@{user_id}> is now registered.",
        timestamp=datetime.utcnow(),
    )

def dm_fail_log_embed(user_id: int, mentor_id: int, support_channel_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.red(),
        description=f"There has been an error sending <@{user_id}> a direct message, "
                    f"they have been directed to ask a <@&{mentor_id}> "
                    f"for assistance in <#{support_channel_id}>.",
        timestamp=datetime.utcnow(),
    )

def name_error_log_embed(user_id: int, mentor_id: int, support_channel_id: int, name: str) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.red(),
        description=f"<@{user_id}> has entered a name that is either invalid or "
                    f"already registered, they have been directed to ask a <@&{mentor_id}> "
                    f"for assistance in <#{support_channel_id}>. The name they provided "
                    f"is **{name}**",
        timestamp=datetime.utcnow(),
    )

def kind_error_log_embed(user_id: int, user_kind: VerifiedUserKind, mentor_id: int, support_channel_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.red(),
        description=f"<@{user_id}> has indicated that the system has incorrectly marked them "
                    f"as a **{user_kind.description()}**, they have been directed to ask a <@&{mentor_id}> "
                    f"for assistance in <#{support_channel_id}>.",
        timestamp=datetime.utcnow(),
    )

def user_left_log_embed(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has left the server and has been deregistered.",
        timestamp=datetime.utcnow(),
    )

def reg_restarted_log_embed(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has restarted the registration process.",
        timestamp=datetime.utcnow(),
    )