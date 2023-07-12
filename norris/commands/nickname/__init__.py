from discord import ApplicationContext, Interaction
from sqlalchemy import select

from ...bot import Norris
from ...model import VerifiedUser


async def handle_nickname(norris: Norris,
                          context: ApplicationContext,
                          new_nickname: str) -> None:
    from ...responses import embeds

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
    )


async def approve_clicked(user_id: int,
                          nickname: str,
                          interaction: Interaction,
                          norris: Norris) -> None:
    # Change the user's nickname
    await norris.get_guild(norris.guild_id).get_member(user_id).edit(nick=nickname)

    from ...responses import embeds

    # Respond with approval
    await interaction.response.send(
        embed=embeds.nickname.approved(nickname),
    )


async def deny_clicked(nickname: str, interaction: Interaction) -> None:
    from ...responses import embeds

    # Respond with denial
    await interaction.response.send(
        embed=embeds.nickname.approved(nickname),
    )
