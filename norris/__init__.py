from asyncio import AbstractEventLoop

from aiomysql.sa import Engine, create_engine
from discord import Bot
from sqlalchemy import URL


class Norris(Bot):
    _token: str
    guild_id: int
    database_engine: Engine

    @staticmethod
    async def run(token: str,
                  guild_id: int,
                  database_url: URL,
                  loop: AbstractEventLoop) -> None:
        norris = Norris()
        norris._token = token
        norris.guild_id = guild_id
        norris.database_engine = await create_engine(user=database_url.username,
                                                     password=database_url.password,
                                                     host=database_url.host,
                                                     db=database_url.database,
                                                     loop=loop)
        await norris.start(norris._token)
