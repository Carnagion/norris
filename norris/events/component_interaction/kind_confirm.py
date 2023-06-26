from discord import Interaction
from sqlalchemy import update

from ...bot import Norris
from ...model import Registration, RegistrationStatus


async def yes_clicked(interaction: Interaction, norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    async with norris.database_engine.begin() as connection:
        # Update the user's registration state to registered
        await connection.execute(
            update(Registration)
            .values(status=RegistrationStatus.REGISTERED)
            .where(Registration.user_id == interaction.user.id),
        )

    # NOTE: thanks Python for this amazing module system
    from ...responses import RegisteredContinueView, kind_confirmed_embed

    # Direct the user to reg support
    await interaction.user.send(
        embed=kind_confirmed_embed(),
        view=RegisteredContinueView(norris),
    )


async def no_clicked(interaction: Interaction, norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    # Update the user's registration state to name entered
    async with norris.database_engine.begin() as connection:
        await connection.execute(
            update(Registration)
            .where(Registration.user_id == interaction.user.id)
            .values(status=RegistrationStatus.NAME_ENTERED),
        )

    # NOTE: I want to bang my head against a wall
    from ...responses import kind_denied_embed

    # Direct the user to reg support
    await interaction.user.send(
        embed=kind_denied_embed(norris.channels.support_channel_id),
    )
