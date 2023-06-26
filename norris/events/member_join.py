from discord import Forbidden, HTTPException, Member
from sqlalchemy import insert

from ..bot import Norris
from ..model import Registration, RegistrationStatus
from ..responses import (
    InstructionsContinueView,
    OpenDirectMessagesView,
    instructions_embed,
    instructions_error_embed,
    instructions_sent_embed,
)


async def on_member_join(member: Member, norris: Norris) -> None:
    # Ignore bots
    if member.bot:
        return

    async with norris.database_engine.begin() as connection:
        # Create new registration state for user
        await connection.execute(
            insert(Registration)
            .values(user_id=member.id,
                    status=RegistrationStatus.UNREGISTERED,
                    name=None,
                    kind=None),
        )

    try:
        # Try sending instructions in DMs
        await member.send(
            embed=instructions_embed(member.id),
            view=InstructionsContinueView(norris),
        )
    except (Forbidden, HTTPException):
        # Inform user if they could not be DMed
        await norris.get_channel(norris.channels.arrival_channel_id).send(
            embed=instructions_error_embed(
                member.id,
                norris.channels.support_channel_id,
            ),
        )
    else:
        # Inform the user of instructions sent to them privately
        await norris.get_channel(norris.channels.arrival_channel_id).send(
            embed=instructions_sent_embed(member.id),
            view=OpenDirectMessagesView(),
        )
