from discord import ApplicationContext
from sqlalchemy import func, select

from ...bot import Norris
from ...model import VerifiedUser, VerifiedUserKind


async def handle_count_postgrads(norris: Norris, context: ApplicationContext) -> None:
    """
    Handles the `count postgrads` slash command.
    """
    # Defer response to allow time for query
    await context.response.defer()

    # Count the total and registered postgrads
    async with norris.database_engine.begin() as connection:
        result = await connection.execute(
            select(func.count(VerifiedUser.registered_user_id), func.count())
            .where(VerifiedUser.kind == VerifiedUserKind.POSTGRAD),
        )
        registered, total = result.one()

    from ...responses import embeds

    # Display results to user
    await context.followup.send(
        embed=embeds.registration.count("Postgraduates", registered, total),
    )
