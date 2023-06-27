from discord import ButtonStyle, Interaction
from discord.ui import Button, View, button

from ..bot import Norris
from ..events.component_interaction import (
    instructions,
    kind_confirm,
    name_confirm,
    pronouns,
    verified,
)


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


class VerifiedContinueView(View):
    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        super().__init__()
        self._norris = norris

    @button(label="Continue", style=ButtonStyle.primary)
    async def continue_clicked(self, _: Button, interaction: Interaction) -> None:
        await verified.continue_clicked(interaction, self._norris)


class PronounsView(View):
    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        super().__init__()
        self._norris = norris

    @button(label="He / Him", style=ButtonStyle.primary)
    async def he_him_clicked(self, _: Button, interaction: Interaction) -> None:
        await pronouns.pronouns_clicked(interaction,
                                        pronouns.Pronouns.HE_HIM,
                                        self._norris)

    @button(label="She / Her", style=ButtonStyle.primary)
    async def she_her_clicked(self, _: Button, interaction: Interaction) -> None:
        await pronouns.pronouns_clicked(interaction,
                                        pronouns.Pronouns.SHE_HER,
                                        self._norris)

    @button(label="They / Them", style=ButtonStyle.primary)
    async def they_them_clicked(self, _: Button, interaction: Interaction) -> None:
        await pronouns.pronouns_clicked(interaction,
                                        pronouns.Pronouns.THEY_THEM,
                                        self._norris)

    @button(label="Xe / Xem", style=ButtonStyle.primary)
    async def xe_xem_clicked(self, _: Button, interaction: Interaction) -> None:
        await pronouns.pronouns_clicked(interaction,
                                        pronouns.Pronouns.XE_XEM,
                                        self._norris)

    @button(label="Any", style=ButtonStyle.primary)
    async def any_clicked(self, _: Button, interaction: Interaction) -> None:
        await pronouns.pronouns_clicked(interaction,
                                        pronouns.Pronouns.ANY,
                                        self._norris)

    @button(label="Ask me", style=ButtonStyle.primary)
    async def ask_clicked(self, _: Button, interaction: Interaction) -> None:
        await pronouns.pronouns_clicked(interaction,
                                        pronouns.Pronouns.ASK,
                                        self._norris)

    @button(label="Skip", style=ButtonStyle.red)
    async def skip_clicked(self, _: Button, interaction: Interaction) -> None:
        await pronouns.skip_clicked(interaction, self._norris)
