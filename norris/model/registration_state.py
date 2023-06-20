from enum import Enum

from sqlalchemy import String
from sqlalchemy.dialects.mysql import BIGINT
from sqlalchemy.orm import Mapped, mapped_column

from .base import DataModel


class RegistrationStatus(Enum):
    UNREGISTERED = "unregistered"
    STARTED = "started"
    NAME_ENTERED = "name_entered"
    NAME_CONFIRMED = "name_confirmed"
    REGISTERED = "registered"
    PRONOUNS_PICKED = "pronouns_picked"
    HOUSING_PICKED = "housing_picked"
    FAILED = "failed"

    @property
    def is_registered(self) -> bool:
        match self:
            case (RegistrationStatus.REGISTERED | RegistrationStatus.PRONOUNS_PICKED
                  | RegistrationStatus.HOUSING_PICKED):
                return True
            case _:
                return False

    def __str__(self) -> str:
        return self.value


class Registration(DataModel):
    __tablename__ = "registrations"

    user_id: Mapped[int] = mapped_column(BIGINT(unsigned=True), primary_key=True)
    status: Mapped[
        RegistrationStatus] = mapped_column(default=RegistrationStatus.UNREGISTERED)

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
    name: Mapped[str] = mapped_column(String(1024),
                                      # NOTE: will be null if the row is another variant
                                      nullable=True,
                                      use_existing_column=True)

    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.NAME_ENTERED,
    }


class NameConfirmed(Registration):
    name: Mapped[str] = mapped_column(String(1024),
                                      # NOTE: will be null if the row is another variant
                                      nullable=True,
                                      use_existing_column=True)

    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.NAME_CONFIRMED,
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
