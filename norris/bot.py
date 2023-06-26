from discord import Bot, Intents
from sqlalchemy.ext.asyncio import AsyncEngine, create_async_engine

from .model import DataModel
from .config import NorrisConfig, RolesConfig, ChannelsConfig


class Norris(Bot):
    database_engine: AsyncEngine
    guild_id: int  # TODO: use when slash commands are setup
    channels: ChannelsConfig
    roles: RolesConfig

    def __init__(self, config: NorrisConfig) -> None:
        # Create bot and connect to database
        super().__init__(intents=Intents.default() | Intents.members)

        # Connect to database
        self.guild_id = config.guild_id
        self.database_engine = create_async_engine(config.database_url, echo=True)

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
