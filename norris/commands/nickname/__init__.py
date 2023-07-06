from discord import ApplicationContext

from ...bot import Norris


async def handle_nickname(norris: Norris,
                          context: ApplicationContext,
                          new_nickname: str) -> None:
    from ...responses import embeds

    # Acknowledge the nickname request
    await context.response.send_message(
        embed=embeds.nickname.acknowledge_request(),
    )
