from typing import Self

from discord import Bot
from sqlalchemy import URL
from sqlalchemy.ext.asyncio import AsyncEngine, create_async_engine

from .events import Events
from .model import DataModel, Registration, VerifiedUser


class Norris(Bot):
    _token: str
    guild_id: int
    database_engine: AsyncEngine

    @staticmethod
    async def create(token: str,
                     guild_id: int,
                     database_url: URL) -> Self:
        # Create bot and connect to database
        norris = Norris()
        norris._token = token
        norris.guild_id = guild_id
        norris.database_engine = create_async_engine(database_url, echo=True)

        # Add commands and event handlers
        norris.add_cog(Events(norris))

        # Set up database and tables
        async with norris.database_engine.connect() as connection:
            await connection.run_sync(DataModel.metadata.create_all)

        return norris

    async def run(self) -> None:
        # Start bot
        await super().start(self._token)

        # Dispose of engine once stopped
        await self.database_engine.dispose()
