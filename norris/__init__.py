from discord import Bot


class Norris(Bot):
    token: str
    guild_id: int

    def __init__(self, token: str, guild_id: int) -> None:
        super().__init__()
        self.token = token
        self.guild_id = guild_id

    def run(self) -> None:
        super().run(self.token)
