from typing import Self

from discord import Bot
from sqlalchemy import URL
from sqlalchemy.ext.asyncio import AsyncEngine, create_async_engine

from .model import DataModel


class Norris(Bot):
    guild_id: int
    database_engine: AsyncEngine
    arrival_channel_id: int
    support_channel_id: int
    log_channel_id: int

    @staticmethod
    async def create(guild_id: int,
                     database_url: URL,
                     arrival_channel_id: int,
                     support_channel_id: int,
                     log_channel_id: int) -> Self:
        # Create bot and connect to database
        norris = Norris()
        norris.guild_id = guild_id
        norris.database_engine = create_async_engine(database_url, echo=True)
        norris.arrival_channel_id = arrival_channel_id
        norris.support_channel_id = support_channel_id
        norris.log_channel_id = log_channel_id

        # NOTE: import done here because fuck Python and fuck its circular imports
        from .events import Events

        # Add commands and event handlers
        norris.add_cog(Events(norris))

        # Set up database and tables
        async with norris.database_engine.connect() as connection:
            await connection.run_sync(DataModel.metadata.create_all)

        return norris

    async def run(self, token: str) -> None:
        # Start bot
        await super().start(token)

        # Dispose of engine once stopped
        await self.database_engine.dispose()
