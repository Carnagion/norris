from discord import Member, RawMemberRemoveEvent
from discord.ext.commands import Cog
from sqlalchemy import delete, update

from . import Norris
from .model import Registration, VerifiedUser


class Events(Cog):
    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        self._norris = norris

    @Cog.listener()
    async def on_member_join(self, member: Member) -> None:
        pass

    # NOTE: we use the raw event so that we don't miss deleting entries from the db
    # in case someone leaves the guild but the cache does not have them
    @Cog.listener()
    async def on_raw_member_remove(self, member_removed: RawMemberRemoveEvent) -> None:
        # FIXME: none of these (or any other queries made here) execute
        async with self._norris.database_engine.begin() as connection:
            await connection.execute(
                delete(Registration)
                .where(Registration.user_id == member_removed.user.id),
            )

            await connection.execute(
                update(VerifiedUser)
                .where(VerifiedUser.registered_user_id == member_removed.user.id)
                .values(registered_user_id=None),
            )
