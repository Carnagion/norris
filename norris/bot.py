from discord import Bot, Intents
from sqlalchemy.ext.asyncio import AsyncEngine, create_async_engine

from .config import ChannelsConfig, NorrisConfig, RolesConfig
from .model import DataModel


class Norris(Bot):
    database_engine: AsyncEngine
    guild_id: int
    channels: ChannelsConfig
    roles: RolesConfig

    def __init__(self, config: NorrisConfig) -> None:
        # Create bot and connect to database
        super().__init__(intents=Intents.default() | Intents.members)

        # Connect to database
        self.guild_id = config.guild_id
        self.channels = config.channels
        self.roles = config.roles
        self.database_engine = create_async_engine(config.database_url, echo=True)

        # NOTE: imports done here because fuck Python and fuck its circular imports
        from .commands import Commands
        from .events import Events

        # Add commands and event handlers
        self.add_cog(Events(self))
        self.add_cog(Commands(self))

    async def run(self, token: str) -> None:
        # Set up database and tables
        async with self.database_engine.connect() as connection:
            await connection.run_sync(DataModel.metadata.create_all)

        # Start bot
        await super().start(token)

        # Dispose of engine once stopped
        await self.database_engine.dispose()
