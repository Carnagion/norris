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
    from ...responses import VerifiedContinueView, verified_continue_embed

    # Inform the user of verification and ask them to continue with optional questions
    await interaction.followup.send(
        embed=verified_continue_embed(),
        view=VerifiedContinueView(norris),
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

    # Ask the user to seek assistance
    await interaction.followup.send(
        embed=kind_error_embed(norris.channels.support_channel_id),
    )
