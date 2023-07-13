"""
Message components used in the registration process and command replies.
"""

from discord import ButtonStyle, Interaction
from discord.ui import Button, View, button

from ..bot import Norris
from ..config import Housing, Pronouns
from ..events.component_interaction import (
    housing,
    instructions,
    kind_confirm,
    name_confirm,
    pronouns,
    verified,
    nickname,
)


class InstructionsContinueView(View):
    """
    A view containing a `Continue` button for starting registration.
    """

    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        super().__init__()
        self._norris = norris

    @button(label="Continue", style=ButtonStyle.primary)
    async def continue_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        A `Continue` button handler for starting registration.
        """
        await instructions.continue_clicked(interaction, self._norris)


class OpenDirectMessagesView(View):
    """
    A view containing a link button that opens direct messages.
    """

    def __init__(self) -> None:
        super().__init__()

        # Add link button to open DMs
        self.add_item(open_dm_button())


def open_dm_button() -> Button:
    """
    A link button that opens direct messages.
    """
    return Button(label="Open direct messages",
                  style=ButtonStyle.link,
                  url="https://discord.com/channels/@me")


class NameConfirmView(View):
    """
    A view containing `Yes` and `No` buttons for confirming a user's name.
    """

    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        super().__init__()
        self._norris = norris

    @button(label="Yes", style=ButtonStyle.green)
    async def yes_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        A `Yes` button handler to confirm a user's name.
        """
        await name_confirm.yes_clicked(interaction, self._norris)

    @button(label="No", style=ButtonStyle.red)
    async def no_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        A `No` button handler to un-confirm a user's name.
        """
        await name_confirm.no_clicked(interaction, self._norris)


class KindConfirmView(View):
    """
    A view containing `Yes` and `No` buttons to confirm a user's
    `norris.model.verified_user.VerifiedUserKind`.
    """

    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        super().__init__()
        self._norris = norris

    @button(label="Yes", style=ButtonStyle.green)
    async def yes_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        A `Yes` button handler to confirm a user's kind.
        """
        await kind_confirm.yes_clicked(interaction, self._norris)

    @button(label="No", style=ButtonStyle.red)
    async def no_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        A `No` button handler to indicate an incorrectly detected kind.
        """
        await kind_confirm.no_clicked(interaction, self._norris)


class VerifiedContinueView(View):
    """
    A view containing a `Continue` button for proceeding past verification to the
    optional questions.
    """

    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        super().__init__()
        self._norris = norris

    @button(label="Continue", style=ButtonStyle.primary)
    async def continue_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        A `Continue` button handler for proceeding past verification to the optional
        questions.
        """
        await verified.continue_clicked(interaction, self._norris)


class PronounsView(View):
    """
    A view containing buttons for selecting (or skipping) pronouns.
    """

    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        super().__init__()
        self._norris = norris

    @button(label="He / Him", style=ButtonStyle.primary)
    async def he_him_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        The `He / Him` pronouns button handler.
        """
        await pronouns.pronouns_clicked(interaction, Pronouns.HE_HIM, self._norris)

    @button(label="She / Her", style=ButtonStyle.primary)
    async def she_her_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        The `She / Her` pronouns button handler.
        """
        await pronouns.pronouns_clicked(interaction, Pronouns.SHE_HER, self._norris)

    @button(label="They / Them", style=ButtonStyle.primary)
    async def they_them_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        The `They / Them` pronouns button handler.
        """
        await pronouns.pronouns_clicked(interaction, Pronouns.THEY_THEM, self._norris)

    @button(label="Xe / Xem", style=ButtonStyle.primary)
    async def xe_xem_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        The `Xe / Xem` pronouns button handler.
        """
        await pronouns.pronouns_clicked(interaction, Pronouns.XE_XEM, self._norris)

    @button(label="Any", style=ButtonStyle.primary)
    async def any_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        The `Any` pronouns button handler.
        """
        await pronouns.pronouns_clicked(interaction, Pronouns.ANY, self._norris)

    @button(label="Ask me", style=ButtonStyle.primary)
    async def ask_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        The `Ask me` pronouns button handler.
        """
        await pronouns.pronouns_clicked(interaction, Pronouns.ASK, self._norris)

    @button(label="Skip", style=ButtonStyle.red)
    async def skip_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        The `Skip` pronouns button handler.
        """
        await pronouns.skip_clicked(interaction, self._norris)


class HousingView(View):
    """
    A view containing buttons for selecting (or skipping) housing.
    """

    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        super().__init__()
        self._norris = norris

    @button(label="Catered halls (Jubilee)", style=ButtonStyle.primary)
    async def jc_catered_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        The `Catered halls (Jubilee)` housing button handler.
        """
        await housing.housing_clicked(interaction, Housing.JC_CATERED, self._norris)

    @button(label="Catered halls (University Park)", style=ButtonStyle.primary)
    async def up_catered_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        The `Catered halls (University Park)` housing button handler.
        """
        await housing.housing_clicked(interaction, Housing.UP_CATERED, self._norris)

    @button(label="Self-catered halls (Jubilee area)", style=ButtonStyle.primary)
    async def jc_self_catered_clicked(self,
                                      _: Button,
                                      interaction: Interaction) -> None:
        """
        The `Self-catered halls (Jubilee)` housing button handler.
        """
        await housing.housing_clicked(interaction,
                                      Housing.JC_SELF_CATERED,
                                      self._norris)

    @button(label="Self-catered halls (University Park area)",
            style=ButtonStyle.primary)
    async def up_self_catered_clicked(self,
                                      _: Button,
                                      interaction: Interaction) -> None:
        """
        The `Self-catered halls (University Park)` housing button handler.
        """
        await housing.housing_clicked(interaction,
                                      Housing.UP_SELF_CATERED,
                                      self._norris)

    @button(label="Private housing", style=ButtonStyle.primary)
    async def private_house_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        The `Private house` housing button handler.
        """
        await housing.housing_clicked(interaction, Housing.PRIVATE_HOUSE, self._norris)

    @button(label="Skip", style=ButtonStyle.red)
    async def skip_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        The `Skip` housing button handler.
        """
        await housing.skip_clicked(interaction, self._norris)


class NicknameView(View):
    """
    A view containing buttons to approve or deny nicknames.
    """

    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        super().__init__()
        self._norris = norris

    @button(label="Approve", style=ButtonStyle.green)
    async def approve_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        The `Approve` nickname button handler.
        """
        user_id = 0  # TODO: figure out which user requested it and their nickname
        new_nickname = ""
        await nickname.approve_clicked(user_id, new_nickname, interaction, self._norris)

    @button(label="Deny", style=ButtonStyle.red)
    async def deny_clicked(self, _: Button, interaction: Interaction) -> None:
        """
        The `Deny` nickname button handler.
        """
        new_nickname = ""  # TODO: figure out how to get the requested nickname
        await nickname.deny_clicked(new_nickname, interaction)
