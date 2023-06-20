import asyncio
import os

import dotenv
import sqlalchemy

from norris import Norris


async def main() -> None:
    dotenv.load_dotenv()

    bot_token = os.getenv("BOT_TOKEN")
    guild_id = int(os.getenv("GUILD_ID"))
    database_url = sqlalchemy.make_url(os.getenv("DATABASE_URL")).set(
        drivername="mysql+asyncmy")

    await Norris.run(bot_token, guild_id, database_url)


asyncio.run(main())
