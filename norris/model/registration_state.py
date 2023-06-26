from enum import Enum

from sqlalchemy import String
from sqlalchemy.dialects.mysql import BIGINT
from sqlalchemy.orm import Mapped, mapped_column

from .base import DataModel
from .verified_user import VerifiedUserKind


class RegistrationStatus(Enum):
    UNREGISTERED = "UNREGISTERED"
    STARTED = "STARTED"
    NAME_ENTERED = "NAME_ENTERED"
    KIND_FOUND = "KIND_FOUND"
    VERIFIED = "VERIFIED"
    PRONOUNS_PICKED = "PRONOUNS_PICKED"
    REGISTERED = "REGISTERED"
    FAILED = "FAILED"

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


class KindFound(Registration):
    name: Mapped[str] = mapped_column(String(1024),
                                      # NOTE: will be null if the row is another variant
                                      nullable=True,
                                      use_existing_column=True)
    kind: Mapped[VerifiedUserKind] = mapped_column(
        # NOTE: will be null if the row is another variant
        nullable=True)

    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.KIND_FOUND,
    }


class Verified(Registration):
    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.VERIFIED,
    }


class PronounsPicked(Registration):
    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.PRONOUNS_PICKED,
    }


class Registered(Registration):
    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.REGISTERED,
    }


class Failed(Registration):
    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.FAILED,
    }
