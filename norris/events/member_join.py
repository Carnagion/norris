from discord import ButtonStyle, Colour, Embed, Forbidden, HTTPException, Member
from discord.ui import Button, View
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
            # Try sending instructions in DMs
            await member.dm_channel.send(
                embed=Embed(
                    title="Registration",
                    description=f"Welcome to the **University of Nottingham Computer "
                                f"Science** server, <@{member.id}>! We'll need a "
                                f"couple of details from you in order to get you set "
                                f"up.",
                    colour=Colour.blurple(),
                ),
            )
        except (Forbidden, HTTPException):
            # Inform user if they could not be DMed
            await norris.get_channel(norris.arrival_channel_id).send(
                embed=Embed(
                    title="Registration",
                    description=f"Welcome to the **University of Nottingham Computer "
                                f"Science** server, <@{member.id}>! Unfortunately, "
                                f"there was an error in sending you instructions. "
                                f"Please seek assistance in <#"
                                f"{norris.support_channel_id}>.",
                    colour=Colour.red(),
                ),
            )
        else:
            # Inform the user of instructions sent to them privately
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


class OpenDirectMessagesView(View):
    def __init__(self) -> None:
        super().__init__()

        # Add link button to open DMs
        self.add_item(Button(label="Open direct messages",
                             style=ButtonStyle.link,
                             url="https://discord.com/channels/@me"))
