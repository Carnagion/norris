import asyncio
# import logging
# from logging import INFO, FileHandler
import os

import dotenv
import sqlalchemy

from norris import Norris


async def main() -> None:
    # Load .env file
    dotenv.load_dotenv()

    # Grab environment variables
    bot_token = os.getenv("BOT_TOKEN")
    guild_id = int(os.getenv("GUILD_ID"))
    database_url = sqlalchemy.make_url(os.getenv("DATABASE_URL")).set(
        drivername="mysql+asyncmy")  # NOTE: the default driver is not async-compatible
    arrival_channel_id = int(os.getenv("ARRIVAL_CHANNEL_ID"))
    support_channel_id = int(os.getenv("SUPPORT_CHANNEL_ID"))
    log_channel_id = int(os.getenv("LOG_CHANNEL_ID"))
    # log_path = os.getenv("LOG_PATH")
    #
    # logging.basicConfig(level=INFO)
    # logging.getLogger().addHandler(FileHandler(log_path))

    # Create and start bot
    norris = Norris(guild_id,
                    database_url,
                    arrival_channel_id,
                    support_channel_id,
                    log_channel_id)
    await norris.run(bot_token)


asyncio.run(main())
