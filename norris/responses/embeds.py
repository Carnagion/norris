from datetime import datetime

from discord import Colour, Embed

from ..model import VerifiedUserKind


def instructions_embed(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"Welcome to the **University of Nottingham Computer Science** "
                    f"server, <@{user_id}>! We'll need a couple of details from you "
                    f"in order to get you set up.",
        timestamp=datetime.utcnow(),
    )


def instructions_sent_embed(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"Welcome to the **University of Nottingham Computer Science** "
                    f"server, <@{user_id}>! Please check your direct messages for "
                    f"instructions on how to continue.",
        timestamp=datetime.utcnow(),
    )


def instructions_error_embed(user_id: int, support_channel_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.red(),
        description=f"Welcome to the **University of Nottingham Computer Science** "
                    f"server, <@{user_id}>! Unfortunately, there was an error in "
                    f"sending you instructions. Please seek assistance in <#"
                    f"{support_channel_id}>.",
        timestamp=datetime.utcnow(),
    )


def request_name_embed() -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description="Please enter your **name** exactly as when you applied to the "
                    "University.",
        timestamp=datetime.utcnow(),
    )


def confirm_name_embed(name: str) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"You entered the name **{name}**. Is that correct?",
        timestamp=datetime.utcnow(),
    )


def no_name_error_embed(support_channel_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.red(),
        description=f"Unfortunately, we don't have that name in our system. Please "
                    f"seek assistance in <#{support_channel_id}>.",
        timestamp=datetime.utcnow(),
    )


def confirm_kind_embed(user_kind: VerifiedUserKind) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"We've detected that you are a **{user_kind.description()}**."
                    f"Is that correct?",
        timestamp=datetime.utcnow(),
    )


def kind_error_embed(support_channel_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.red(),
        description=f"Unfortunately, our system might have assigned you incorrectly."
                    f"Please seek assistance in <#{support_channel_id}>.",
        timestamp=datetime.utcnow(),
    )


def verified_continue_embed() -> Embed:
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


def pronouns_embed() -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description="What are your **pronouns**? Displaying your pronouns is "
                    "important for making everyone on the server feel comfortable, "
                    "they are for everyone regardless of if you are cis or under the "
                    "trans umbrella. If you use multiple pronouns, or want to change "
                    "your pronouns later, you can change them at any time in the "
                    "pronouns channel once you are registered.",
        timestamp=datetime.utcnow(),
    )


def housing_embed() -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description="What kind of **accommodation** will you be staying in? This "
                    "helps you find others staying in the same accommodation or "
                    "similar types, and will only be displayed via a role on your "
                    "profile.",
        timestamp=datetime.utcnow(),
    )


def registration_finished_embed(chat_channel_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.green(),
        description=f"Thank you for your patience! Your registration is now "
                    f"**complete**. We have no additional questions. You can head "
                    f"over to <#{chat_channel_id}> to chat with your new course peers "
                    f"and mentors.",
        timestamp=datetime.utcnow(),
    )
