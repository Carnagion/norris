from discord import Interaction

from ...bot import Norris


async def continue_clicked(interaction: Interaction, norris: Norris) -> None:
    # TODO: this will be useful for later
    await interaction.response.defer()

    # NOTE: I hate circular imports
    from ...responses import PronounsView, pronouns_embed

    # Ask the user their pronouns
    await interaction.followup.send(
        embed=pronouns_embed(),
        view=PronounsView(norris),
    )
