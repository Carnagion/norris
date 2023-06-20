from asyncio import AbstractEventLoop

from aiomysql.sa import Engine, create_engine
from discord import Bot
from sqlalchemy import URL
from sqlalchemy.schema import CreateTable

from .model import DataModel, Registration, VerifiedUser


class Norris(Bot):
    _token: str
    guild_id: int
    database_engine: Engine

    @staticmethod
    async def run(token: str,
                  guild_id: int,
                  database_url: URL,
                  loop: AbstractEventLoop) -> None:
        # Create bot and connect to database
        norris = Norris()
        norris._token = token
        norris.guild_id = guild_id
        # TODO: find an appropriate value for max connections
        norris.database_engine = await create_engine(user=database_url.username,
                                                     password=database_url.password,
                                                     host=database_url.host,
                                                     db=database_url.database,
                                                     loop=loop,
                                                     maxsize=25)

        # Set up database and tables
        await norris._setup_db()

        # Start bot
        await norris.start(norris._token)

    async def _setup_db(self) -> None:
        async with self.database_engine.acquire() as connection:
            for table in [VerifiedUser.__table__, Registration.__table__]:
                await connection.execute(CreateTable(table, if_not_exists=True))
