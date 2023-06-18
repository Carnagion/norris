import asyncio
import os
from asyncio import AbstractEventLoop

import dotenv
import sqlalchemy

from norris import Norris


async def main(loop: AbstractEventLoop) -> None:
    dotenv.load_dotenv()

    bot_token = os.getenv("BOT_TOKEN")
    guild_id = int(os.getenv("GUILD_ID"))
    database_url = os.getenv("DATABASE_URL")

    await Norris.run(bot_token,
                     guild_id,
                     sqlalchemy.make_url(database_url),
                     loop)


main_loop = asyncio.get_event_loop()  # FIXME: figure out why the hell this causes a deprecation warning
main_loop.run_until_complete(main(main_loop))
