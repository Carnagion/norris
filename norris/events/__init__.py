from discord import Member, RawMemberRemoveEvent, Embed, Colour
from discord.ext.commands import Cog
from sqlalchemy import delete, insert, update

from ..bot import Norris
from ..model import Registration, RegistrationStatus, VerifiedUser


class Events(Cog):
    _norris: Norris

    def __init__(self, norris: Norris) -> None:
        self._norris = norris

    @Cog.listener()
    async def on_member_join(self, member: Member) -> None:
        # Ignore bots
        if member.bot:
            return

        async with self._norris.database_engine.begin() as connection:
            # Create new registration state for user
            await connection.execute(
                insert(Registration)
                .values(user_id=member.id, status=RegistrationStatus.UNREGISTERED),
            )

            await self._norris.get_channel(self._norris.arrival_channel_id).send(
                embed=Embed(
                    title="Registration",
                    description=f"Welcome to the **University of Nottingham Computer "
                                f"Science** server, <@{member.id}>! Please check your "
                                f"direct messages for instructions on how to continue.",
                    colour=Colour.blurple(),
                ),
            )

    # NOTE: we use the raw event so that we don't miss deleting entries from the db
    # in case someone leaves the guild but the cache does not have them
    @Cog.listener()
    async def on_raw_member_remove(self, member_removed: RawMemberRemoveEvent) -> None:
        # Ignore bots
        if member_removed.user.bot:
            return

        # FIXME: none of these (or any other queries made here) execute
        async with self._norris.database_engine.begin() as connection:
            # Remove ongoing registration state of user
            await connection.execute(
                delete(Registration)
                .where(Registration.user_id == member_removed.user.id),
            )

            # Set their registered user ID to null if they were registered
            await connection.execute(
                update(VerifiedUser)
                .where(VerifiedUser.registered_user_id == member_removed.user.id)
                .values(registered_user_id=None),
            )
