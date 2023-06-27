from enum import Enum

from discord import Interaction
from sqlalchemy import update

from ...bot import Norris
from ...model import Registration, RegistrationStatus


class Pronouns(Enum):
    HE_HIM = "he_him"
    SHE_HER = "she_her"
    THEY_THEM = "they_them"
    XE_XEM = "xe_xem"
    ANY = "any"
    ASK = "ask"

    def role(self, norris: Norris) -> int:
        pronouns_config = norris.roles.pronouns
        match self:
            case Pronouns.HE_HIM:
                return pronouns_config.he_him_role_id
            case Pronouns.SHE_HER:
                return pronouns_config.she_her_role_id
            case Pronouns.THEY_THEM:
                return pronouns_config.they_them_role_id
            case Pronouns.XE_XEM:
                return pronouns_config.xe_xem_role_id
            case Pronouns.ANY:
                return pronouns_config.any_pronouns_role_id
            case Pronouns.ASK:
                return pronouns_config.ask_pronouns_role_id


async def pronouns_clicked(interaction: Interaction,
                           pronouns: Pronouns,
                           norris: Norris) -> None:
    # Give the user the desired pronouns role
    # FIXME: need to get a Member from a User so we can add roles
    await interaction.user.add_roles(pronouns.role(norris))

    # Move on to housing
    await skip_clicked(interaction, norris)


async def skip_clicked(interaction: Interaction, norris: Norris) -> None:
    # Update the user's registration state to pronouns picked
    async with norris.database_engine.begin() as connection:
        await connection.execute(
            update(Registration)
            .values(status=RegistrationStatus.PRONOUNS_PICKED)
            .where(Registration.user_id == interaction.user.id),
        )

    from ...responses import housing_embed

    # Ask the user to pick housing
    interaction.followup.send(embed=housing_embed())
