from enum import Enum

from sqlalchemy import String
from sqlalchemy.dialects.mysql import BIGINT
from sqlalchemy.orm import Mapped, mapped_column

from .base import DataModel
from .verified_user import VerifiedUserKind


class RegistrationStatus(Enum):
    """
    States of the registration process.
    """

    UNREGISTERED = "UNREGISTERED"
    """
    The user has not started registration yet.

    Users enter this stage when joining the guild or when restarting their registration.
    """

    STARTED = "STARTED"
    """
    The user has started registration.
    """

    NAME_ENTERED = "NAME_ENTERED"
    """
    The user has entered (but not yet confirmed) their name.
    """

    KIND_FOUND = "KIND_FOUND"
    """
    The user has confirmed their name, and at least one unregistered
    `norris.model.verified_user.VerifiedUser` with the same name is found.
    """

    VERIFIED = "VERIFIED"
    """
    The user has confirmed their `norris.model.verified_user.VerifiedUserKind` and is
    verified, but not registered.
    """

    PRONOUNS_PICKED = "PRONOUNS_PICKED"
    """
    The user has picked their pronouns (or skipped the question).
    """

    REGISTERED = "REGISTERED"
    """
    The user has finished registration.
    """

    FAILED = "FAILED"
    """
    One of the previous registration steps has failed.

    Causes of failure include:

    - The user cannot be sent a direct message to start (or restart) registration
    - A matching unregistered `norris.model.verified_user.VerifiedUser` entry could
    not be found
    - The user indicates that their matched
    `norris.model.verified_user.VerifiedUserKind` is incorrect
    """

    def __str__(self) -> str:
        return self.value


class Registration(DataModel):
    """
    Registration data of a user.
    """

    __tablename__ = "registrations"

    user_id: Mapped[int] = mapped_column(BIGINT(unsigned=True), primary_key=True)
    """
    The user's Discord ID.
    """

    status: Mapped[
        RegistrationStatus] = mapped_column(default=RegistrationStatus.UNREGISTERED)
    """
    The user's registration status, represented as a `RegistrationStatus`.
    """

    __mapper_args__ = {
        "polymorphic_on": status,
    }


class Unregistered(Registration):
    """
    State held by a user who has not started (or has restarted) registration.
    """

    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.UNREGISTERED,
    }


class Started(Registration):
    """
    State held by a user who has started registration.
    """

    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.STARTED,
    }


class NameEntered(Registration):
    """
    State held by a user who has entered but not confirmed their name.
    """

    name: Mapped[str] = mapped_column(String(1024),
                                      # NOTE: will be null if the row is another variant
                                      nullable=True,
                                      use_existing_column=True)
    """
    Name entered by the user.
    """

    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.NAME_ENTERED,
    }


class KindFound(Registration):
    """
    State held by a user who has confirmed their name, and at least one unregistered
    `norris.model.verified_user.VerifiedUser` with the same name is found.
    """

    name: Mapped[str] = mapped_column(String(1024),
                                      # NOTE: will be null if the row is another variant
                                      nullable=True,
                                      use_existing_column=True)
    """
    Name entered by the user.
    """

    kind: Mapped[VerifiedUserKind] = mapped_column(
        # NOTE: will be null if the row is another variant
        nullable=True)
    """
    The detected user kind.
    """

    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.KIND_FOUND,
    }


class Verified(Registration):
    """
    State held by a user who has confirmed their
    `norris.model.verified_user.VerifiedUserKind` and is verified, but not registered.
    """

    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.VERIFIED,
    }


class PronounsPicked(Registration):
    """
    State held by a user who has picked their pronouns (or skipped the question).
    """

    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.PRONOUNS_PICKED,
    }


class Registered(Registration):
    """
    State held by a user who has completed registration.
    """

    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.REGISTERED,
    }


class Failed(Registration):
    """
    State held by a user who has failed one of the previous registration steps.
    """

    __mapper_args__ = {
        "polymorphic_identity": RegistrationStatus.FAILED,
    }
