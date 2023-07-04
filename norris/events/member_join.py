from discord import Forbidden, HTTPException, Member
from sqlalchemy import insert

from ..bot import Norris
from ..model import Registration, RegistrationStatus
from ..responses import embeds
from ..responses.components import (
    InstructionsContinueView,
    OpenDirectMessagesView,
)


async def on_member_join(member: Member, norris: Norris) -> None:
    # Ignore bots
    if member.bot:
        return

    # Log user joining
    await norris.get_channel(norris.channels.log_channel_id).send(
        embed=embeds.logs.user_joined(member.id),
    )

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
            embed=embeds.registration.instructions(member.id),
            view=InstructionsContinueView(norris),
        )
    except (Forbidden, HTTPException):
        # Inform user if they could not be DMed
        await norris.get_channel(norris.channels.arrival_channel_id).send(
            embed=embeds.registration.instructions_error(
                member.id,
                norris.channels.support_channel_id,
            ),
        )

        # Alert mentors about the error
        await norris.get_channel(norris.channels.log_channel_id).send(
            embed=embeds.logs.dm_error(
                member.id,
                norris.roles.hierarchy.mentor_role_id,
                norris.channels.support_channel_id,
            ),
        )
    else:
        # Inform the user of instructions sent to them privately
        await norris.get_channel(norris.channels.arrival_channel_id).send(
            embed=embeds.registration.instructions_sent(member.id),
            view=OpenDirectMessagesView(),
        )
