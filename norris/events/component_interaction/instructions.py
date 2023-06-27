from discord import Interaction
from sqlalchemy import update

from ...bot import Norris
from ...model import Registration, RegistrationStatus


async def continue_clicked(interaction: Interaction, norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    # Update the user's registration state to started
    async with norris.database_engine.begin() as connection:
        await connection.execute(
            update(Registration)
            .values(status=RegistrationStatus.STARTED)
            .where(Registration.user_id == interaction.user.id),
        )

    # NOTE: I love circular imports, what an amazing module system Python has
    from ...responses import request_name_embed, reg_started_log_embed

    # Ask the user to enter their name
    await interaction.followup.send(embed=request_name_embed())
    await norris.get_channel(norris.channels.log_channel_id).send(
        embed=reg_started_log_embed(interaction.user.id),
    )
