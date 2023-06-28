from sqlalchemy import select

from ...bot import Norris
from ...model import Registration, RegistrationStatus


async def verify_registration_status(user_id: int,
                                     against: RegistrationStatus,
                                     norris: Norris) -> bool:
    async with norris.database_engine.begin() as connection:
        result = await connection.execute(
            select(Registration)
            .where(Registration.user_id == user_id),
        )
        return result.one().status == against
