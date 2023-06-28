from discord import Interaction
from sqlalchemy import select, update

from ...bot import Norris
from ...model import KindFound, Registration, RegistrationStatus, VerifiedUser


async def yes_clicked(interaction: Interaction, norris: Norris) -> None:
    registration = None

    # Defer response to give time for database queries
    await interaction.response.defer()
    
    async with norris.database_engine.begin() as connection:
        # Get the user's name and kind
        result = await connection.execute(
            select(KindFound)
            .where(Registration.user_id == interaction.user.id)
            .limit(1),
        )
        registration = result.one()

        # Update the user's registration state to verified
        await connection.execute(
            update(Registration)
            .values(status=RegistrationStatus.VERIFIED, name=None, kind=None)
            .where(Registration.user_id == interaction.user.id),
        )

        # Select the ID of a verified user with a matching name and role
        # NOTE: we need this because sqlalchemy doesn't support limits on updates, which
        # means we can't just limit the change to one row like in the Rust version
        result = await connection.execute(
            select(VerifiedUser)
            .where(VerifiedUser.name == registration.name
                   and VerifiedUser.kind == registration.kind
                   and VerifiedUser.registered_user_id is None)
            .limit(1),
        )
        verified_user = result.one()

        # Link the user's account to the matching verified user
        await connection.execute(
            update(VerifiedUser)
            .values(registered_user_id=interaction.user.id)
            .where(VerifiedUser.id == verified_user.id),
        )

    # NOTE: thanks Python for this amazing module system
    from ...responses import VerifiedContinueView, verified_continue_embed, role_confirmed_log_embed, verified_log_embed

    # Inform the user of verification and ask them to continue with optional questions
    await interaction.followup.send(
        embed=verified_continue_embed(),
        view=VerifiedContinueView(norris),
    )
    await norris.get_channel(norris.channels.log_channel_id).send(
        embed=role_confirmed_log_embed(interaction.user.id, registration.kind),
    )
    await norris.get_channel(norris.channels.log_channel_id).send(
        embed=verified_log_embed(interaction.user.id),
    )


async def no_clicked(interaction: Interaction, norris: Norris) -> None:
    registration = None

    # Defer response to give time for database queries
    await interaction.response.defer()

    # Get kind listed in database
    async with norris.database_engine.begin() as connection:
        # Get the user's name and kind
        result = await connection.execute(
            select(KindFound)
            .where(Registration.user_id == interaction.user.id)
            .limit(1),
        )
        registration = result.one()

    # Update the user's registration state to failed
    async with norris.database_engine.begin() as connection:
        await connection.execute(
            update(Registration)
            .values(status=RegistrationStatus.FAILED, name=None, kind=None)
            .where(Registration.user_id == interaction.user.id),
        )

    

    # NOTE: I want to bang my head against a wall
    from ...responses import kind_error_embed, kind_error_log_embed

    # Ask the user to seek assistance
    await interaction.followup.send(
        embed=kind_error_embed(norris.channels.support_channel_id),
    )
    await norris.get_channel(norris.channels.log_channel_id).send(
        embed=kind_error_log_embed(interaction.user.id, registration.kind,
                                   norris.roles.hierarchy.mentor_role_id, norris.channels.support_channel_id),
    )
