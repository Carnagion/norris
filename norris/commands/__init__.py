from discord import SlashCommandGroup, ApplicationContext, Role
from discord.ext.commands import Cog, has_guild_permissions

from ..bot import Norris
from .registration import nuke  # , restart


class Commands(Cog):
    _norris: Norris
    registration = SlashCommandGroup("registration", "Registration commands")

    def __init__(self, norris: Norris) -> None:
        super().__init__()
        self._norris = norris

    @registration.command()
    @has_guild_permissions(administrator=True)
    async def nuke(self, context: ApplicationContext, role: Role | None = None) -> None:
        await nuke.nuke(self._norris, context, role)

    # @registration.command()
    # @has_guild_permissions(administrator=True)
    # async def restart(self, context) -> None:
    #     await restart.restart(self._norris, context)
