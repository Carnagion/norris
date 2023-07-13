"""
Handlers for the `nickname` command.
"""

from discord import ApplicationContext
from sqlalchemy import select

from ...bot import Norris
from ...model import VerifiedUser


async def handle_nickname(norris: Norris,
                          context: ApplicationContext,
                          new_nickname: str) -> None:
    """
    Handles the `nickname` command.
    """
    from ...responses import embeds
    from ...responses.components import NicknameView

    # Acknowledge the nickname request
    await context.response.send_message(
        embed=embeds.nickname.acknowledge_request(),
    )

    async with norris.database_engine.begin() as connection:
        # Grab the user's name
        result = await connection.execute(
            select(VerifiedUser)
            .where(VerifiedUser.registered_user_id == context.author.id)
            .limit(1),
        )
        name = result.one().name

    # Ask mentors to approve or deny nickname
    await norris.get_channel(norris.channels.nickname_channel_id).send(
        embed=embeds.nickname.request_approval(
            context.author.id,
            name,
            context.user.nick,
            new_nickname,
        ),
        view=NicknameView(norris),
    )

