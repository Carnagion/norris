from discord import Member, Message, RawMemberRemoveEvent
from discord.ext.commands import Cog

from ..bot import Norris
from . import member_join, message, raw_member_remove


class Events(Cog):
    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        self._norris = norris

    @Cog.listener()
    async def on_member_join(self, member: Member) -> None:
        await member_join.on_member_join(member, self._norris)

    # NOTE: we use the raw event so that we don't miss deleting entries from the db
    # in case someone leaves the guild but the cache does not have them
    @Cog.listener()
    async def on_raw_member_remove(self, member_removed: RawMemberRemoveEvent) -> None:
        await raw_member_remove.on_raw_member_remove(member_removed, self._norris)

    @Cog.listener()
    async def on_message(self, received_message: Message) -> None:
        await message.on_message(received_message, self._norris)

