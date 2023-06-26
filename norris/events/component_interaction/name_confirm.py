from discord import Interaction
from sqlalchemy import select, update

from ...bot import Norris
from ...model import NameEntered, Registration, RegistrationStatus, VerifiedUser


async def yes_clicked(interaction: Interaction, norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    async with norris.database_engine.begin() as connection:
        # Retrieve the user's name
        result = await connection.execute(
            select(NameEntered)
            .where(Registration.user_id == interaction.user.id)
            .limit(1),
        )

        user_name = result.one().name  # NOTE: this should be a NameEntered

        # Try to find a matching user who is not registered
        result = await connection.execute(
            select(VerifiedUser)
            .where(VerifiedUser.name == user_name
                   and VerifiedUser.registered_user_id is None)
            .order_by(VerifiedUser.kind)
            .limit(1),
        )
        verified_user = result.one_or_none()

        from ...responses import (
            KindConfirmView,
            confirm_kind_embed,
            no_name_error_embed,
        )

        if verified_user is None:
            # Update the user's registration state to failed
            await connection.execute(
                update(Registration)
                .values(status=RegistrationStatus.FAILED, name=None)
                .where(Registration.user_id == interaction.user.id),
            )

            # Ask the user to seek assistance
            await interaction.followup.send(
                embed=no_name_error_embed(norris.channels.support_channel_id),
            )
        else:
            # Update the user's registration state to kind found
            await connection.execute(
                update(Registration)
                .where(Registration.user_id == interaction.user.id)
                .values(
                    status=RegistrationStatus.KIND_FOUND,
                ),
            )

            # Ask the user to confirm their kind
            user_kind = verified_user.kind.description()
            await interaction.user.send(
                embed=confirm_kind_embed(user_kind),
                view=KindConfirmView(norris),
            )


async def no_clicked(interaction: Interaction, norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    # Update the user's registration state to started
    async with norris.database_engine.begin() as connection:
        await connection.execute(
            update(Registration)
            .where(Registration.user_id == interaction.user.id)
            .values(status=RegistrationStatus.STARTED, name=None),
        )

    # NOTE: circular modules again
    from ...responses import request_name_embed

    # Ask the user to enter their name
    await interaction.followup.send(embed=request_name_embed())
