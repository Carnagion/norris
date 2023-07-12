"""
Embeds used in messages related to the registration process.
"""

from datetime import datetime

from discord import Colour, Embed

from ...model import VerifiedUserKind


def instructions(user_id: int) -> Embed:
    """
    Embed for displaying registration instructions to a user.
    """
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"Welcome to the **University of Nottingham Computer Science** "
                    f"server, <@{user_id}>! We'll need a couple of details from you "
                    f"in order to get you set up.",
        timestamp=datetime.utcnow(),
    )


def instructions_sent(user_id: int) -> Embed:
    """
    Embed for notifying a user to check their direct messages for instructions.
    """
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"Welcome to the **University of Nottingham Computer Science** "
                    f"server, <@{user_id}>! Please check your direct messages for "
                    f"instructions on how to continue.",
        timestamp=datetime.utcnow(),
    )


def instructions_error(user_id: int, support_channel_id: int) -> Embed:
    """
    Embed for notifying a user of errors while sending registration instructions.
    """
    return Embed(
        title="Registration",
        colour=Colour.red(),
        description=f"Welcome to the **University of Nottingham Computer Science** "
                    f"server, <@{user_id}>! Unfortunately, there was an error in "
                    f"sending you instructions. Please seek assistance in <#"
                    f"{support_channel_id}>.",
        timestamp=datetime.utcnow(),
    )


def name_request() -> Embed:
    """
    Embed for requesting a user to enter their name.
    """
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description="Please enter your **name** exactly as when you applied to the "
                    "University.",
        timestamp=datetime.utcnow(),
    )


def name_confirm(name: str) -> Embed:
    """
    Embed for requesting a user to confirm their name.
    """
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"You entered the name **{name}**. Is that correct?",
        timestamp=datetime.utcnow(),
    )


def name_error(support_channel_id: int) -> Embed:
    """
    Embed for when a name is not in the database or is already registered.
    """
    return Embed(
        title="Registration",
        colour=Colour.red(),
        description=f"Unfortunately, we don't have that name in our system. Please "
                    f"seek assistance in <#{support_channel_id}>.",
        timestamp=datetime.utcnow(),
    )


def kind_confirm(user_kind: VerifiedUserKind) -> Embed:
    """
    Embed for requesting a user to confirm their
    `norris.model.verified_user.VerifiedUserKind`.
    """
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"We've detected that you are a **{user_kind.description()}**. Is "
                    f"that correct?",
        timestamp=datetime.utcnow(),
    )


def kind_error(support_channel_id: int) -> Embed:
    """
    Embed for when a `norris.model.verified_user.VerifiedUserKind` is detected
    incorrectly.
    """
    return Embed(
        title="Registration",
        colour=Colour.red(),
        description=f"Unfortunately, our system might have assigned you incorrectly."
                    f"Please seek assistance in <#{support_channel_id}>.",
        timestamp=datetime.utcnow(),
    )


def verified() -> Embed:
    """
    Embed for when a user is verified.
    """
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description="Thank you for your patience! You are now **verified**, but not "
                    "fully registered yet. We still have a few questions for you "
                    "regarding your pronouns and accommodation. Please note that you "
                    "can skip these if you wish to. Your registration will be "
                    "complete after you answer or skip the following questions.",
        timestamp=datetime.utcnow(),
    )


def pronouns() -> Embed:
    """
    Embed for asking a user their pronouns.
    """
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description="What are your **pronouns**? Displaying your pronouns is "
                    "important for making everyone on the server feel comfortable - "
                    "they are for everyone regardless of if you are cis or under the "
                    "trans umbrella. If you use multiple pronouns, or want to change "
                    "your pronouns later, you can change them at any time in the "
                    "pronouns channel once you are registered.",
        timestamp=datetime.utcnow(),
    )


def housing() -> Embed:
    """
    Embed for asking a user their accommodation.
    """
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description="What kind of **accommodation** will you be staying in? This "
                    "helps you find others staying in the same accommodation or "
                    "similar types, and will only be displayed via a role on your "
                    "profile.",
        timestamp=datetime.utcnow(),
    )


def finished(chat_channel_id: int) -> Embed:
    """
    Embed for indicating registration completion.
    """
    return Embed(
        title="Registration",
        colour=Colour.green(),
        description=f"Thank you for your patience! Your registration is now "
                    f"**complete**. We have no additional questions. You can head "
                    f"over to <#{chat_channel_id}> to chat with your new course peers "
                    f"and mentors.",
        timestamp=datetime.utcnow(),
    )


def welcome(user_id: int) -> Embed:
    """
    Embed for welcoming a user after their registration is complete.
    """
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"Welcome to the server, <@{user_id}>!",
        timestamp=datetime.utcnow(),
    )


def restart(user_id: int) -> Embed:
    """
    Embed for indicating the restart of a user's registration.
    """
    return Embed(
        title="Registration",
        colour=Colour(0xFAA81A),  # NOTE: official Discord warning colour
        description=f"Re-started <@{user_id}>'s registration.",
        timestamp=datetime.utcnow(),
    )


def nuke() -> Embed:
    """
    Embed for indicating that a registration nuke occurred.
    """
    return Embed(
        title="Registration",
        colour=Colour(0xFAA81A),  # NOTE: official Discord warning colour
        description="Nuked registrations.",
        timestamp=datetime.utcnow(),
    )
