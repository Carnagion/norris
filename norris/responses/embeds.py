from discord import Colour, Embed


def instructions_embed(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"Welcome to the **University of Nottingham Computer Science** "
                    f"server, <@{user_id}>!\nWe'll need a couple of details from you "
                    f"in order to get you set up.",
    )


def instructions_sent_embed(user_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"Welcome to the **University of Nottingham Computer Science** "
                    f"server, <@{user_id}>!\nPlease check your direct messages for "
                    f"instructions on how to continue.",
    )


def instructions_error_embed(user_id: int, support_channel_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.red(),
        description=f"Welcome to the **University of Nottingham Computer Science** "
                    f"server, <@{user_id}>!\nUnfortunately, there was an error in "
                    f"sending you instructions. Please seek assistance in <#"
                    f"{support_channel_id}>.",
    )


def request_name_embed() -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description="Please enter your **name** exactly as when you applied to the "
                    "University.",
    )


def confirm_name_embed(name: str) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"You entered the name **{name}**. Is that correct?",
    )


def no_name_error_embed(support_channel_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.red(),
        description=f"Unfortunately, we don't have that name in our system. Please "
                    f"seek assistance in <#{support_channel_id}>.",
    )

def confirm_kind_embed(user_kind: str) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"Our system has detected you are a **{user_kind}**, is that "
                    f"correct?",
    )

def kind_denied_embed(support_channel_id: int) -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description=f"We're having trouble finding your role, please seek assistance "
                    f"in <#{support_channel_id}>.",
    )

def kind_confirmed_embed() -> Embed:
    return Embed(
        title="Registration",
        colour=Colour.blurple(),
        description="Thank you for your patience!\nYour role is now **confirmed**.\n"
                    "We still have a few questions for you regarding your pronouns and "
                    "accommodation.\nPlease note that these are completely optional "
                    "and you can skip them if you wish to.",
    )