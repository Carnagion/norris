from discord import ApplicationContext, Member, Option, Role, SlashCommandGroup
from discord.ext import commands
from discord.ext.commands import Cog, has_guild_permissions

from ..bot import Norris
from .nickname import handle_nickname
from .registration import handle_nuke, handle_restart


class Commands(Cog):
    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        super().__init__()
        self._norris = norris

    registration = SlashCommandGroup("registration", "Registration commands")

    @registration.command()
    @has_guild_permissions(administrator=True)
    async def nuke(self,
                   context: ApplicationContext,
                   role: Option(Role) = None) -> None:
        await handle_nuke(self._norris, context, role)

    @registration.command()
    @has_guild_permissions(administrator=True)
    async def restart(self,
                      context: ApplicationContext,
                      member: Option(Member)) -> None:
        await handle_restart(self._norris, context, member)

    # FIXME: pycord complains about not being able to remove commands
    @commands.command()
    async def nickname(self, context: ApplicationContext, nickname: str) -> None:
        await handle_nickname(self._norris, context, nickname)
