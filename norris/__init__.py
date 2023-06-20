from discord import Bot
from sqlalchemy import URL
from sqlalchemy.ext.asyncio import create_async_engine, AsyncEngine

from .model import DataModel, Registration, VerifiedUser


class Norris(Bot):
    _token: str
    guild_id: int
    database_engine: AsyncEngine

    @staticmethod
    async def run(token: str,
                  guild_id: int,
                  database_url: URL) -> None:
        # Create bot and connect to database
        norris = Norris()
        norris._token = token
        norris.guild_id = guild_id
        norris.database_engine = create_async_engine(database_url, echo=True)

        # Set up database and tables
        await norris._setup_db()

        # Start bot
        await norris.start(norris._token)

        # Dispose of database connection when bot stops
        await norris.database_engine.dispose()

    async def _setup_db(self) -> None:
        async with self.database_engine.connect() as connection:
            await connection.run_sync(DataModel.metadata.create_all)
