import asyncio

from norris import Norris, NorrisConfig


async def main() -> None:
    with open("norris.toml") as file:  # NOQA: ASYNC101 - synchronous io here is ok
        toml = file.read()
    config = NorrisConfig.from_toml(toml)

    # Create and start bot
    norris = Norris(config)
    await norris.run(config.bot_token)


asyncio.run(main())
