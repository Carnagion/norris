from discord import Interaction

from ...bot import Norris


async def continue_clicked(interaction: Interaction, norris: Norris) -> None:
    # NOTE: I hate circular imports
    from ...responses import pronouns_embed, PronounsView

    # Ask the user their pronouns
    await interaction.response.send(
        embed=pronouns_embed(),
        view=PronounsView(norris),
    )
