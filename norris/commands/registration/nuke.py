from ...bot import Norris
from sqlalchemy import select, update
from ...model import Registration, RegistrationStatus, VerifiedUser, VerifiedUserKind
from discord import Forbidden, HTTPException

async def nuke(norris: Norris, context) -> None:
    # await context.respond(f"Hello, <@{context.author.id}>!") # Have only this uncommented to test command permissions

    # USE WITH CAUTION - WILL NUKE SERVER IF TESTING DONE WITH WRONG CODE UNCOMMENTED
    guild = norris.get_guild(norris.guild_id)
    member_list = guild.fetch_members()
    async for member in member_list:
        sm_role = guild.get_role(norris.roles.hierarchy.senior_mentor_role_id)
        hm_role = guild.get_role(norris.roles.hierarchy.honorary_mentor_role_id)
        fac_role = guild.get_role(norris.roles.hierarchy.faculty_role_id)
        cr_role = guild.get_role(1117785827053948976)

        if not(sm_role in member.roles or hm_role in member.roles or 
               fac_role in member.roles or member.bot): # COMMENT THIS OUT WHEN TESTING
        # if cr_role in member.roles: # Use if you want to test nuke - only nukes course reps
            await member.edit(roles=[])
            async with norris.database_engine.begin() as connection:
                # Set their registered user ID to null if they were registered
                await connection.execute(
                    update(VerifiedUser)
                    .where(VerifiedUser.registered_user_id == member.id)
                    .values(registered_user_id=None),
                )

                await connection.execute(
                    update(Registration)
                    .where(Registration.user_id == member.id)
                    .values(status = RegistrationStatus.UNREGISTERED, name = None, kind = None),
                )
            
            from ...responses import instructions_embed, InstructionsContinueView, instructions_error_embed
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