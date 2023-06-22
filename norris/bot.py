from typing import Self

from discord import Bot, Intents
from sqlalchemy import URL
from sqlalchemy.ext.asyncio import AsyncEngine, create_async_engine

from .model import DataModel


class Norris(Bot):
    guild_id: int
    database_engine: AsyncEngine
    arrival_channel_id: int
    support_channel_id: int
    log_channel_id: int

    def __init__(self,
                 guild_id: int,
                 database_url: URL,
                 arrival_channel_id: int,
                 support_channel_id: int,
                 log_channel_id: int) -> None:
        # Create bot and connect to database
        super().__init__(intents=Intents.default() | Intents.members)

        self.guild_id = guild_id
        self.database_engine = create_async_engine(database_url, echo=True)
        self.arrival_channel_id = arrival_channel_id
        self.support_channel_id = support_channel_id
        self.log_channel_id = log_channel_id

        # NOTE: import done here because fuck Python and fuck its circular imports
        from .events import Events

        # Add commands and event handlers
        self.add_cog(Events(self))

    async def run(self, token: str) -> None:
        # Set up database and tables
        async with self.database_engine.connect() as connection:
            await connection.run_sync(DataModel.metadata.create_all)

        # Start bot
        await super().start(token)

        # Dispose of engine once stopped
        await self.database_engine.dispose()
