from discord import ButtonStyle, Interaction
from discord.ui import Button, View, button

from ..bot import Norris
from ..events import component_interaction


class InstructionsContinueView(View):
    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        super().__init__()
        self._norris = norris

    @button(label="Continue", style=ButtonStyle.primary)
    async def instructions_continue_clicked(self,
                                            _: Button,
                                            interaction: Interaction) -> None:
        await component_interaction.on_instructions_continue_clicked(interaction,
                                                                     self._norris)


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
    async def name_confirmed(self,
                             _: Button,
                             interaction: Interaction) -> None:
        # TODO: add logic for handling name confirmation
        pass

    @button(label="No", style=ButtonStyle.red)
    async def name_denied(self,
                          _: Button,
                          interaction: Interaction) -> None:
        await component_interaction.name_denied(interaction,
                                                self._norris)
