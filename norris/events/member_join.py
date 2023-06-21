from discord import Colour, Embed, Member, ButtonStyle, HTTPException, Forbidden
from discord.ui import View, Button
from sqlalchemy import insert

from ..bot import Norris
from ..model import Registration, RegistrationStatus


async def on_member_join(norris: Norris, member: Member) -> None:
    # Ignore bots
    if member.bot:
        return

    async with norris.database_engine.begin() as connection:
        # Create new registration state for user
        await connection.execute(
            insert(Registration)
            .values(user_id=member.id, status=RegistrationStatus.UNREGISTERED),
        )

        try:
            await norris.get_channel(norris.arrival_channel_id).send(
                embed=Embed(
                    title="Registration",
                    description=f"Welcome to the **University of Nottingham Computer "
                                f"Science** server, <@{member.id}>! Please check your "
                                f"direct messages for instructions on how to continue.",
                    colour=Colour.blurple(),
                ),
                view=OpenDirectMessagesView(),
            )
        except (Forbidden, HTTPException):
            pass


class OpenDirectMessagesView(View):
    def __init__(self) -> None:
        super().__init__()

        # Add link button to open DMs
        self.add_item(Button(label="Open direct messages",
                             style=ButtonStyle.link,
                             url="https://discord.com/channels/@me"))
