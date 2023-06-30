from sqlalchemy import update
from discord import Forbidden, HTTPException, ApplicationContext, Role

from ...bot import Norris
from ...model import Registration, RegistrationStatus, VerifiedUser


async def nuke(norris: Norris,
               context: ApplicationContext,
               role: Role | None = None) -> None:
    # await context.respond(f"Hello, <@{context.author.id}>!") # Have only this
    # uncommented to test command permissions (no nuking occurs)

    guild = norris.get_guild(norris.guild_id)

    roles = norris.roles.nukeable_role_ids() if role is None else [role.id]

    # USE WITH CAUTION - WILL NUKE SERVER IF TESTING DONE WITH WRONG CODE UNCOMMENTED
    async for member in guild.fetch_members():
        # Ignore member if they do not have any nukeable role
        if not any(map(lambda role: role.id in roles, member.roles)):
            return

        # Remove all roles from the member
        await member.edit(roles=[])

        async with norris.database_engine.begin() as connection:
            # Set their registered user ID to null if they were registered
            await connection.execute(
                update(VerifiedUser)
                .values(registered_user_id=None)
                .where(VerifiedUser.registered_user_id == member.id),
            )

            # Update their registration state to unregistered
            await connection.execute(
                update(Registration)
                .values(status=RegistrationStatus.UNREGISTERED, name=None, kind=None)
                .where(Registration.user_id == member.id),
            )

        from ...responses import instructions_embed, InstructionsContinueView, \
            instructions_error_embed

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

    await context.respond("Nuke in progress!")
