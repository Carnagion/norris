from discord import Interaction

from ...bot import Norris


async def approve_clicked(user_id: int,
                          nickname: str,
                          interaction: Interaction,
                          norris: Norris) -> None:
    """
    Called when a mentor approves a user's nickname request.
    """
    # Change the user's nickname
    await norris.get_guild(norris.guild_id).get_member(user_id).edit(nick=nickname)

    from ...responses import embeds

    # Respond with approval
    await interaction.response.send(
        embed=embeds.nickname.approved(nickname),
    )


async def deny_clicked(nickname: str, interaction: Interaction) -> None:
    """
    Called when a mentor denies a user's nickname request.
    """
    from ...responses import embeds

    # Respond with denial
    await interaction.response.send(
        embed=embeds.nickname.approved(nickname),
    )