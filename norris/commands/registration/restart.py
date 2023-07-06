from discord import ApplicationContext, Member
from sqlalchemy import update

from ...bot import Norris
from ...model import Registration, RegistrationStatus, VerifiedUser


async def handle_restart(norris: Norris,
                         context: ApplicationContext,
                         member: Member) -> None:
    # Defer response to give time for database queries
    await context.response.defer()

    # Ignore bots
    if member.bot:
        return

    # Restart the user's registration process
    await restart_registration(member, norris)

    from ...responses import embeds

    # Reply after restarting their registration
    await context.followup.send(embed=embeds.registration.restart(member.id))


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

    from ...events.member_join import try_send_instructions
    from ...responses import embeds

    # Log the registration restart
    await norris.get_channel(norris.channels.log_channel_id).send(
        embed=embeds.logs.registration_restarted(member.id),
    )

    # Ask the user to start registration again
    await try_send_instructions(member, norris)
