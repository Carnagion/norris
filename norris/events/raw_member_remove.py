from discord import RawMemberRemoveEvent
from sqlalchemy import delete, update

from ..bot import Norris
from ..model import Registration, VerifiedUser


async def on_raw_member_remove(member_removed: RawMemberRemoveEvent,
                               norris: Norris) -> None:
    # Ignore bots
    if member_removed.user.bot:
        return

    async with norris.database_engine.begin() as connection:
        # Set their registered user ID to null if they were registered
        await connection.execute(
            update(VerifiedUser)
            .where(VerifiedUser.registered_user_id == member_removed.user.id)
            .values(registered_user_id=None),
        )

        # Remove ongoing registration state of user
        await connection.execute(
            delete(Registration)
            .where(Registration.user_id == member_removed.user.id),
        )

    from ..responses import embeds

    # Log user leaving
    await norris.get_channel(norris.channels.log_channel_id).send(
        embed=embeds.logs.user_left(member_removed.user.id),
    )
