from discord import Interaction

from ...bot import Norris
from ...model import RegistrationStatus
from . import verify_registration_status


async def continue_clicked(interaction: Interaction, norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    # Check that the user has the correct state to click the button
    if not await verify_registration_status(interaction.user.id,
                                            RegistrationStatus.VERIFIED,
                                            norris):
        return

    # NOTE: I hate circular imports
    from ...responses import embeds
    from ...responses.components import PronounsView

    # Ask the user their pronouns
    await interaction.followup.send(
        embed=embeds.registration.pronouns_embed(),
        view=PronounsView(norris),
    )
