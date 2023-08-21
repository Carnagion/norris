from discord import Interaction
from sqlalchemy import update

from ...bot import Norris
from ...config import Pronouns
from ...model import Registration, RegistrationStatus
from . import verify_registration_status


async def pronouns_clicked(interaction: Interaction,
                           pronouns: Pronouns,
                           norris: Norris) -> None:
    """
    Called when a user picks a pronoun.
    """
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
    """
    Called when a user skips pronouns.
    """
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
    """
    Transitions to the housing selection stage.
    """
    async with norris.database_engine.begin() as connection:
        # Update the user's registration state to pronouns picked
        await connection.execute(
            update(Registration)
            .values(status=RegistrationStatus.PRONOUNS_PICKED)
            .where(Registration.user_id == interaction.user.id),
        )

    # NOTE: circular imports
    from ...responses import embeds
    from ...responses.components import HousingView

    # Ask the user to pick housing
    await interaction.followup.send(
        embed=embeds.registration.housing(),
        view=HousingView(norris),
    )

    # Log selection of pronouns
    await norris.get_channel(norris.channels.log_channel_id).send(
        embed=embeds.logs.pronouns_picked(interaction.user.id),
    )
