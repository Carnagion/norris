"""
Event handlers for message component interactions.
"""

from sqlalchemy import select

from ...bot import Norris
from ...model import Registration, RegistrationStatus


async def verify_registration_status(user_id: int,
                                     against: RegistrationStatus,
                                     norris: Norris) -> bool:
    """
    Check if a user has a specific `norris.model.registration_state.RegistrationStatus`.
    """
    async with norris.database_engine.begin() as connection:
        result = await connection.execute(
            select(Registration)
            .where(Registration.user_id == user_id),
        )
        return result.one().status == against
