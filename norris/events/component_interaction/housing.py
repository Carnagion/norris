from discord import Interaction
from sqlalchemy import select, update

from ...bot import Norris
from ...config import Housing
from ...model import Registration, RegistrationStatus, VerifiedUser


async def housing_clicked(interaction: Interaction,
                          housing: Housing,
                          norris: Norris) -> None:
    # Give the user the desired housing role
    guild = norris.get_guild(norris.guild_id)
    member = guild.get_member(interaction.user.id)
    await member.add_roles(norris.roles.housing.role_id(housing))

    # Move on to housing
    await skip_clicked(interaction, norris)


async def skip_clicked(interaction: Interaction, norris: Norris) -> None:
    async with norris.database_engine.begin() as connection:
        # Update the user's registration state to registered
        await connection.execute(
            update(Registration)
            .values(status=RegistrationStatus.REGISTERED)
            .where(Registration.user_id == interaction.user.id),
        )

        # Retrieve the user's kind
        result = await connection.execute(
            select(VerifiedUser)
            .where(VerifiedUser.registered_user_id == interaction.user.id)
            .limit(1),
        )
        verified_user = result.one()

        # Give the user the relevant hierarchy role
        guild = norris.get_guild(norris.guild_id)
        member = guild.get_member(interaction.user.id)
        await member.add_roles(norris.roles.hierarchy.role_id(verified_user.kind))

    from ...responses import registration_finished_embed

    # Inform the user of completion
    interaction.followup.send(
        embed=registration_finished_embed(norris.channels.chat_channel_id),
    )
