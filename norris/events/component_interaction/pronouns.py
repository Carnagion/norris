from discord import Interaction
from sqlalchemy import update

from ...bot import Norris
from ...config import Pronouns
from ...model import Registration, RegistrationStatus
from . import verify_registration_status


async def pronouns_clicked(interaction: Interaction,
                           pronouns: Pronouns,
                           norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    # Check that the user has the correct state to click the button
    if not await verify_registration_status(interaction.user.id,
                                            RegistrationStatus.VERIFIED,
                                            norris):
        return

    # Give the user the desired pronouns role
    guild = norris.get_guild(norris.guild_id)
    member = guild.get_member(interaction.user.id)
    role = guild.get_role(norris.roles.pronouns.role_id(pronouns))
    await member.add_roles(role)

    # Move on to housing
    await transition_to_housing(interaction, norris)


async def skip_clicked(interaction: Interaction, norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    # Check that the user has the correct state to click the button
    if not await verify_registration_status(interaction.user.id,
                                            RegistrationStatus.VERIFIED,
                                            norris):
        return

    # Move on to housing
    await transition_to_housing(interaction, norris)


async def transition_to_housing(interaction: Interaction, norris: Norris) -> None:
    async with norris.database_engine.begin() as connection:
        # Update the user's registration state to pronouns picked
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

    # Log selection of pronouns
    await norris.get_channel(norris.channels.log_channel_id).send(
        embed=pronouns_selected_log_embed(interaction.user.id),
    )
