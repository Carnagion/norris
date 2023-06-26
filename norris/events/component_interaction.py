from discord import Interaction
from sqlalchemy import select, update

from ..bot import Norris
from ..model import NameEntered, Registration, RegistrationStatus, VerifiedUser


async def on_instructions_continue_clicked(interaction: Interaction,
                                           norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    # Update the user's registration state to started
    async with norris.database_engine.begin() as connection:
        await connection.execute(
            update(Registration)
            .where(Registration.user_id == interaction.user.id)
            .values(status=RegistrationStatus.STARTED),
        )

    # NOTE: I love circular imports, what an amazing module system Python has
    from ..responses import request_name_embed

    # Ask the user to enter their name
    await interaction.followup.send(embed=request_name_embed())


async def name_confirmed(interaction: Interaction, norris: Norris) -> None:
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

        match verified_user:
            case None:
                await no_name_error(interaction, norris)
            case VerifiedUser():
                await connection.execute(
                    update(Registration)
                    .where(Registration.user_id == interaction.user.id)
                    .values(
                        status=RegistrationStatus.KIND_FOUND,
                    ),
                )

                # NOTE: sigh
                from ..responses import confirm_kind_embed, KindConfirmView

                user_kind = verified_user.kind.description()
                await interaction.user.send(
                    embed=confirm_kind_embed(user_kind),
                    view=KindConfirmView(norris),
                )


async def name_denied(interaction: Interaction, norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    # Update the user's registration state to started
    async with norris.database_engine.begin() as connection:
        await connection.execute(
            update(Registration)
            .where(Registration.user_id == interaction.user.id)
            .values(status=RegistrationStatus.STARTED, name=None),
        )

    # NOTE: guess what
    from ..responses import request_name_embed

    # Ask the user to enter their name
    await interaction.followup.send(embed=request_name_embed())


async def no_name_error(interaction: Interaction, norris: Norris) -> None:
    async with norris.database_engine.begin() as connection:
        # Update the user's registration state to failed
        await connection.execute(
            update(Registration)
            .values(status=RegistrationStatus.FAILED, name=None)
            .where(Registration.user_id == interaction.user.id),
        )

    # NOTE: circular modules again
    from ..responses import no_name_error_embed

    # Ask the user to seek assistance
    await interaction.followup.send(
        embed=no_name_error_embed(norris.channels.support_channel_id),
    )


async def kind_denied(interaction: Interaction, norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    # Update the user's registration state to name entered
    async with norris.database_engine.begin() as connection:
        await connection.execute(
            update(Registration)
            .where(Registration.user_id == interaction.user.id)
            .values(status=RegistrationStatus.NAME_ENTERED),
        )

    # NOTE: guess what
    from ..responses import kind_denied_embed

    # Direct the user to reg support
    await interaction.user.send(
        embed=kind_denied_embed(norris.channels.support_channel_id),
    )


async def kind_confirmed(interaction: Interaction, norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    async with norris.database_engine.begin() as connection:
        # Update the user's registration state to registered
        await connection.execute(
            update(Registration)
            .values(status=RegistrationStatus.REGISTERED)
            .where(Registration.user_id == interaction.user.id),
        )

    # NOTE: guess what
    from ..responses import kind_confirmed_embed, RegisteredContinueView

    # Direct the user to reg support
    await interaction.user.send(
        embed=kind_confirmed_embed(),
        view=RegisteredContinueView(norris),
    )
