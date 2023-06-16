from dataclasses import dataclass

from discord import Bot


@dataclass()
class Norris(Bot):
    _token: str
    _guild_id: int

    def run(self) -> None:
        super().run(self._token)
