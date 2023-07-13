"""
Slash command definitions and handlers.
"""

from discord import ApplicationContext, Member, Option, Role, SlashCommandGroup
from discord.ext import commands
from discord.ext.commands import Cog, has_guild_permissions

from ..bot import Norris
from .nickname import handle_nickname
from .registration import handle_nuke, handle_restart


class Commands(Cog):
    """
    Defines and handles all slash commands recognised by the bot.
    """

    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        super().__init__()
        self._norris = norris

    registration = SlashCommandGroup("registration", "Modify registrations.")
    """
    Registration-related slash commands.
    """

    @registration.command()
    @has_guild_permissions(administrator=True)
    async def nuke(self,
                   context: ApplicationContext,
                   role: Option(Role) = None) -> None:
        """
        Restart the registrations of multiple users.
        """
        await handle_nuke(self._norris, context, role)

    @registration.command()
    @has_guild_permissions(administrator=True)
    async def restart(self,
                      context: ApplicationContext,
                      member: Option(Member)) -> None:
        """
        Restart a user's registration.
        """
        await handle_restart(self._norris, context, member)

    # FIXME: pycord complains about not being able to remove commands
    @commands.command()
    async def nickname(self, context: ApplicationContext, nickname: str) -> None:
        await handle_nickname(self._norris, context, nickname)
