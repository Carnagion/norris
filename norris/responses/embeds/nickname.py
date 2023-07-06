from datetime import datetime

from discord import Colour, Embed, EmbedField


def acknowledge_request() -> Embed:
    return Embed(
        title="Nickname request",
        colour=Colour.blurple(),
        description="Your nickname request has been received. Please wait for "
                    "approval by a mentor.",
        timestamp=datetime.utcnow(),
    )


def request_approval(user_id: int,
                     name: str,
                     current_nickname: str,
                     requested_nickname: str) -> Embed:
    return Embed(
        title="Nickname request",
        colour=Colour.blurple(),
        description=f"<@{user_id}> has requested a new nickname.",
        fields=[
            EmbedField("Current nickname", current_nickname, inline=True),
            EmbedField("Requested nickname", requested_nickname, inline=True),
            EmbedField("Verified name", name),
        ],
        timestamp=datetime.utcnow(),
    )


def approved(nickname: str) -> Embed:
    return Embed(
        title="Nickname request",
        colour=Colour.green(),
        description=f"Your request to change your nickname to **{nickname}** was "
                    f"**approved**.",
        timestamp=datetime.utcnow(),
    )


def denied(nickname: str) -> Embed:
    return Embed(
        title="Nickname request",
        colour=Colour.red(),
        description=f"Your request to change your nickname to **{nickname}** was "
                    f"**denied**. Please note that you can message a mentor if you "
                    f"think this was a mistake.",
        timestamp=datetime.utcnow(),
    )
