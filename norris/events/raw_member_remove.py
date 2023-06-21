from discord import RawMemberRemoveEvent
from sqlalchemy import delete, update

from ..bot import Norris
from ..model import Registration, VerifiedUser


async def on_raw_member_remove(norris: Norris,
                               member_removed: RawMemberRemoveEvent) -> None:
    # Ignore bots
    if member_removed.user.bot:
        return

    # FIXME: none of these (or any other queries made here) execute
    async with norris.database_engine.begin() as connection:
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
