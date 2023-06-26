import asyncio

from norris import Norris, NorrisConfig


async def main() -> None:
    with open("norris.toml", "r") as file:
        toml = file.read()
    config = NorrisConfig.from_toml(toml)

    # Create and start bot
    norris = Norris(config)
    await norris.run(config.bot_token)


asyncio.run(main())
