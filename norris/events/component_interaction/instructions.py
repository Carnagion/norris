from discord import Interaction
from sqlalchemy import update

from ...bot import Norris
from ...model import Registration, RegistrationStatus
from . import verify_registration_status


async def continue_clicked(interaction: Interaction, norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    # Check that the user has the correct state to click the button
    if not await verify_registration_status(interaction.user.id,
                                            RegistrationStatus.UNREGISTERED,
                                            norris):
        return

    async with norris.database_engine.begin() as connection:
        # Update the user's registration state to started
        await connection.execute(
            update(Registration)
            .values(status=RegistrationStatus.STARTED)
            .where(Registration.user_id == interaction.user.id),
        )

    # NOTE: I love circular imports, what an amazing module system Python has
    from ...responses import embeds

    # Ask the user to enter their name
    await interaction.followup.send(embed=embeds.registration.request_name_embed())

    # Log the start of registration
    await norris.get_channel(norris.channels.log_channel_id).send(
        embed=embeds.logs.registration_started_log_embed(interaction.user.id),
    )
