from discord import Bot

import dotenv

import os

dotenv.load_dotenv()

DISCORD_TOKEN: str = os.getenv("DISCORD_TOKEN")
GUILD_ID: int = int(os.getenv("GUILD_ID"))

norris = Bot()
norris.run(DISCORD_TOKEN)
