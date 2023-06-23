from discord import Message
from sqlalchemy import select, update

from ..bot import Norris
from ..model import Registration, RegistrationStatus
from ..responses import NameConfirmView, confirm_name_embed


async def on_message(message: Message, norris: Norris) -> None:
    # Ignore bots
    if message.author.bot:
        return

    async with norris.database_engine.begin() as connection:
        # Try to get the user's registration status
        result = await connection.execute(
            select(Registration)
            .where(Registration.user_id == message.author.id),
        )
        registration = result.scalars().one_or_none()

        # Ignore if the user has not registered
        if registration is None:
            return

        # Update the status to name entered if they have started registration and
        # entered a name
        if message.content and registration.status == RegistrationStatus.STARTED:
            await connection.execute(
                update(Registration)
                .where(Registration.user_id == message.author.id)
                .values(
                    status=RegistrationStatus.NAME_ENTERED,
                    name=message.content,
                ),
            )

    # Ask user to confirm their name
    await message.channel.send(
        embed=confirm_name_embed(message.content),
        view=NameConfirmView(norris),
        reference=message,
    )
