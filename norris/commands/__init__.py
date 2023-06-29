from discord import SlashCommandGroup
from discord.ext.commands import Cog, has_guild_permissions

from ..bot import Norris

from .registration import nuke, restart

class Commands(Cog):
    _norris: Norris
    def __init__(self, norris: Norris) -> None:
        self._norris = norris

    registration = SlashCommandGroup("registration", "Registration commands")

    @registration.command()
    @has_guild_permissions(administrator=True)
    async def nuke(self, context) -> None:
        await nuke.nuke(self._norris, context)

    @registration.command()
    @has_guild_permissions(administrator=True)
    async def restart(self, context) -> None:
        await restart.restart(self._norris, context)