from discord import Interaction
from sqlalchemy import update

from ...bot import Norris
from ...config import Pronouns
from ...model import Registration, RegistrationStatus


async def pronouns_clicked(interaction: Interaction,
                           pronouns: Pronouns,
                           norris: Norris) -> None:
    # Give the user the desired pronouns role
    guild = norris.get_guild(norris.guild_id)
    member = guild.get_member(interaction.user.id)
    role = guild.get_role(norris.roles.pronouns.role_id(pronouns))
    await member.add_roles(role)

    # Move on to housing
    await skip_clicked(interaction, norris)


async def skip_clicked(interaction: Interaction, norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    # Update the user's registration state to pronouns picked
    async with norris.database_engine.begin() as connection:
        await connection.execute(
            update(Registration)
            .values(status=RegistrationStatus.PRONOUNS_PICKED)
            .where(Registration.user_id == interaction.user.id),
        )

    from ...responses import HousingView, housing_embed, pronouns_selected_log_embed
    # Ask the user to pick housing
    await interaction.followup.send(
        embed=housing_embed(),
        view=HousingView(norris),
    )
    await norris.get_channel(norris.channels.log_channel_id).send(
        embed=pronouns_selected_log_embed(interaction.user.id),
    )
