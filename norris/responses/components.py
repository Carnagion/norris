from discord import ButtonStyle, Interaction
from discord.ui import Button, View, button

from ..bot import Norris
from ..events.component_interaction import instructions, kind_confirm, name_confirm


class InstructionsContinueView(View):
    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        super().__init__()
        self._norris = norris

    @button(label="Continue", style=ButtonStyle.primary)
    async def continue_clicked(self, _: Button, interaction: Interaction) -> None:
        await instructions.continue_clicked(interaction, self._norris)


class OpenDirectMessagesView(View):
    def __init__(self) -> None:
        super().__init__()

        # Add link button to open DMs
        self.add_item(open_dm_button())


def open_dm_button() -> Button:
    return Button(label="Open direct messages",
                  style=ButtonStyle.link,
                  url="https://discord.com/channels/@me")


class NameConfirmView(View):
    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        super().__init__()
        self._norris = norris

    @button(label="Yes", style=ButtonStyle.green)
    async def yes_clicked(self, _: Button, interaction: Interaction) -> None:
        await name_confirm.yes_clicked(interaction, self._norris)

    @button(label="No", style=ButtonStyle.red)
    async def no_clicked(self, _: Button, interaction: Interaction) -> None:
        await name_confirm.no_clicked(interaction, self._norris)


class KindConfirmView(View):
    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        super().__init__()
        self._norris = norris

    @button(label="Yes", style=ButtonStyle.green)
    async def yes_clicked(self, _: Button, interaction: Interaction) -> None:
        await kind_confirm.yes_clicked(interaction, self._norris)

    @button(label="No", style=ButtonStyle.red)
    async def no_clicked(self, _: Button, interaction: Interaction) -> None:
        await kind_confirm.no_clicked(interaction, self._norris)


class RegisteredContinueView(View):
    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        super().__init__()
        self._norris = norris

    @button(label="Continue", style=ButtonStyle.primary)
    async def registered_continue_clicked(self,
                                          _: Button,
                                          interaction: Interaction) -> None:
        pass
        """await component_interaction.on_instructions_continue_clicked(interaction,
                                                                     self._norris)"""
