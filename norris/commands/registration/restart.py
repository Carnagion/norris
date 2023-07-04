from discord import ApplicationContext, Forbidden, HTTPException, Role, Member
from sqlalchemy import update

from ...bot import Norris
from ...model import Registration, RegistrationStatus, VerifiedUser


async def restart(norris: Norris,
                  context: ApplicationContext,
                  member: Member) -> None:
    await restart_registration(member, norris)

    await context.respond()


async def restart_registration(member: Member, norris: Norris) -> None:
    async with norris.database_engine.begin() as connection:
        # Update their registration state to unregistered
        await connection.execute(
            update(Registration)
            .values(status=RegistrationStatus.UNREGISTERED, name=None, kind=None)
            .where(Registration.user_id == member.id),
        )

        # Set their registered user ID to null if they were registered
        await connection.execute(
            update(VerifiedUser)
            .values(registered_user_id=None)
            .where(VerifiedUser.registered_user_id == member.id),
        )

    # Remove all registration-related roles from the member
    roles = map(norris.get_guild(norris.guild_id).get_role,
                norris.roles.role_ids_needing_registration())
    await member.remove_roles(*roles)

    from ...responses.components import InstructionsContinueView
    from ...responses import embeds

    try:
        # Try sending instructions in DMs
        await member.send(
            embed=embeds.registration.instructions_embed(member.id),
            view=InstructionsContinueView(norris),
        )
    except (Forbidden, HTTPException):
        # Inform user if they could not be DMed
        await norris.get_channel(norris.channels.arrival_channel_id).send(
            embed=embeds.registration.instructions_error_embed(
                member.id,
                norris.channels.support_channel_id,
            ),
        )
