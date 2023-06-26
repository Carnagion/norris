from discord import Interaction
from sqlalchemy import update

from ...bot import Norris
from ...model import Registration, RegistrationStatus


async def yes_clicked(interaction: Interaction, norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    async with norris.database_engine.begin() as connection:
        # Update the user's registration state to verified
        await connection.execute(
            update(Registration)
            .values(status=RegistrationStatus.VERIFIED, name=None, kind=None)
            .where(Registration.user_id == interaction.user.id),
        )

    # NOTE: thanks Python for this amazing module system
    from ...responses import RegisteredContinueView, verified_continue_embed

    # Direct the user to reg support
    await interaction.user.send(
        embed=verified_continue_embed(),
        view=RegisteredContinueView(norris),
    )


async def no_clicked(interaction: Interaction, norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    # Update the user's registration state to failed
    async with norris.database_engine.begin() as connection:
        await connection.execute(
            update(Registration)
            .values(status=RegistrationStatus.FAILED, name=None, kind=None)
            .where(Registration.user_id == interaction.user.id),
        )

    # NOTE: I want to bang my head against a wall
    from ...responses import kind_error_embed

    # Direct the user to reg support
    await interaction.user.send(
        embed=kind_error_embed(norris.channels.support_channel_id),
    )
