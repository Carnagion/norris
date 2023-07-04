from discord import Interaction
from sqlalchemy import select, update

from ...bot import Norris
from ...config import Housing
from ...model import Registration, RegistrationStatus, VerifiedUser, VerifiedUserKind
from . import verify_registration_status


async def housing_clicked(interaction: Interaction,
                          housing: Housing,
                          norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    # Check that the user has the correct state to click the button
    if not await verify_registration_status(interaction.user.id,
                                            RegistrationStatus.PRONOUNS_PICKED,
                                            norris):
        return

    # Give the user the desired housing role
    guild = norris.get_guild(norris.guild_id)
    member = guild.get_member(interaction.user.id)
    role = guild.get_role(norris.roles.housing.role_id(housing))
    await member.add_roles(role)

    # Finish registration
    await finish_registration(interaction, norris)


async def skip_clicked(interaction: Interaction, norris: Norris) -> None:
    # Defer response to give time for database queries
    await interaction.response.defer()

    # Check that the user has the correct state to click the button
    if not await verify_registration_status(interaction.user.id,
                                            RegistrationStatus.PRONOUNS_PICKED,
                                            norris):
        return

    # Finish registration
    await finish_registration(interaction, norris)


async def finish_registration(interaction: Interaction, norris: Norris) -> None:
    async with norris.database_engine.begin() as connection:
        # Update the user's registration state to registered
        await connection.execute(
            update(Registration)
            .values(status=RegistrationStatus.REGISTERED)
            .where(Registration.user_id == interaction.user.id),
        )

        # Retrieve the user's kind
        result = await connection.execute(
            select(VerifiedUser)
            .where(VerifiedUser.registered_user_id == interaction.user.id)
            .limit(1),
        )
        verified_user = result.one()

        # Give the user the relevant hierarchy role
        guild = norris.get_guild(norris.guild_id)
        member = guild.get_member(interaction.user.id)
        role = guild.get_role(norris.roles.hierarchy.role_id(verified_user.kind))
        await member.add_roles(role)

        # Change the user's nickname to their verified name
        await member.edit(nick=verified_user.name)

    from ...responses import embeds

    # Find the correct atrium channel for undergrads and postgrads
    is_postgrad = verified_user.kind == VerifiedUserKind.POSTGRAD
    undergrad_main = norris.channels.undergrad.main_channel_id
    postgrad_main = norris.channels.postgrad.main_channel_id
    main_channel_id = postgrad_main if is_postgrad else undergrad_main

    # Inform the user of completion
    await interaction.followup.send(
        embed=embeds.registration.finished(main_channel_id),
    )

    # Welcome the user in their corresponding atrium
    await norris.get_channel(main_channel_id).send(
        embed=embeds.registration.welcome(interaction.user.id),
    )

    # Log completion of registration
    await norris.get_channel(norris.channels.log_channel_id).send(
        embeds=[embeds.logs.housing_picked(interaction.user.id),
                embeds.logs.registration_finished(interaction.user.id)],
    )
