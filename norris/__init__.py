from dataclasses import dataclass

from discord import Bot


@dataclass()
class Norris(Bot):
    token: str
    guild_id: int

    def run(self) -> None:
        super().run(self.token)
