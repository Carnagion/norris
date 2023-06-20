from enum import Enum

from sqlalchemy import Enum as SqlEnum
from sqlalchemy import String
from sqlalchemy.orm import Mapped, mapped_column

from .base import DataModel
from .verified_user import VerifiedUserKind


class RegistrationStatus(Enum):
    UNREGISTERED = 0
    STARTED = 1
    NAME_ENTERED = 2
    NAME_CONFIRMED = 3
    KIND_CONFIRMED = 4
    REGISTERED = 5
    PRONOUNS_PICKED = 6
    HOUSING_PICKED = 7
    FAILED = -1

    @property
    def is_registered(self) -> bool:
        return self >= RegistrationStatus.REGISTERED


class Registration(DataModel):
    __tablename__ = "registrations"

    user_id: Mapped[int] = mapped_column(primary_key=True)
    status: Mapped[RegistrationStatus] = mapped_column(SqlEnum(RegistrationStatus),
                                                       default=RegistrationStatus.UNREGISTERED)

    __mapper_args__ = {
        "polymorphic_on": status,
    }


class Unregistered(Registration):
    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.UNREGISTERED,
    }


class Started(Registration):
    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.STARTED,
    }


class NameEntered(Registration):
    name: Mapped[str] = mapped_column(String(1024), use_existing_column=True)

    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.NAME_ENTERED,
    }


class NameConfirmed(Registration):
    name: Mapped[str] = mapped_column(String(1024), use_existing_column=True)

    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.NAME_CONFIRMED,
    }


class KindConfirmed(Registration):
    name: Mapped[str] = mapped_column(String(1024), use_existing_column=True)
    kind: Mapped[VerifiedUserKind] = mapped_column(SqlEnum(VerifiedUserKind))

    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.KIND_CONFIRMED,
    }


class Registered(Registration):
    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.REGISTERED,
    }


class PronounsPicked(Registration):
    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.PRONOUNS_PICKED,
    }


class HousingPicked(Registration):
    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.HOUSING_PICKED,
    }


class Failed(Registration):
    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.FAILED,
    }
