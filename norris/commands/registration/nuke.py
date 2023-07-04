from discord import ApplicationContext, Role

from ...bot import Norris
from .restart import restart_registration


async def nuke(norris: Norris,
               context: ApplicationContext,
               role: Role | None = None) -> None:
    # Defer response to give time for database queries
    await context.response.defer()

    guild = norris.get_guild(norris.guild_id)

    roles = norris.roles.nukeable_role_ids() if role is None else [role.id]

    # USE WITH CAUTION - WILL NUKE SERVER IF TESTING DONE WITH CODE UNCOMMENTED
    async for member in guild.fetch_members():
        # Ignore member if they are a bot or do not have any nukeable role
        if member.bot or not any(role.id in roles for role in member.roles):
            return

        # Restart the member's registration
        await restart_registration(member, norris)

    from ...responses import embeds

    # Reply after nuke
    await context.followup.send(embed=embeds.registration.nuke())
