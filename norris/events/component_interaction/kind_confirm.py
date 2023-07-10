from discord import Interaction
from sqlalchemy import select, update

from ...bot import Norris
from ...model import KindFound, Registration, RegistrationStatus, VerifiedUser
from . import verify_registration_status


async def yes_clicked(interaction: Interaction, norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    # Check that the user has the correct state to click the button
    if not await verify_registration_status(interaction.user.id,
                                            RegistrationStatus.KIND_FOUND,
                                            norris):
        return

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
    from ...responses import embeds
    from ...responses.components import VerifiedContinueView

    # Inform the user of verification and ask them to continue with optional questions
    await interaction.followup.send(
        embed=embeds.registration.verified(),
        view=VerifiedContinueView(norris),
    )

    # Log the verification
    await norris.get_channel(norris.channels.log_channel_id).send(
        embed=embeds.logs.verified(interaction.user.id, verified_user.kind),
    )


async def no_clicked(interaction: Interaction, norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    # Check that the user has the correct state to click the button
    if not await verify_registration_status(interaction.user.id,
                                            RegistrationStatus.KIND_FOUND,
                                            norris):
        return

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
    from ...responses import embeds

    # Ask the user to seek assistance
    await interaction.followup.send(
        embed=embeds.registration.kind_error(norris.channels.support_channel_id),
    )

    # Alert mentors about kind error
    await norris.get_channel(norris.channels.log_channel_id).send(
        content=f"<@&{norris.roles.hierarchy.mentor_role_id}>",
        embed=embeds.logs.kind_error(interaction.user.id,
                                     registration.kind,
                                     norris.channels.support_channel_id),
    )
