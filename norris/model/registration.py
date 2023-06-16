from enum import Enum

from typing import Optional

from .member import MemberKind


# I would die for enums like Haskell's or Rust's, but unfortunately this is Python
class RegistrationStatus(Enum):
    UNREGISTERED = 0
    STARTED = 1
    NAME_ENTERED = 2
    NAME_CONFIRMED = 3
    KIND_CONFIRMED = 4
    PRONOUNS_PICKED = 5
    HOUSING_PICKED = 6
    FAILED = -1

    @property
    def is_registered(self) -> bool:
        # NOTE: a user is considered registered as long as their name and kind is
        # verified, even if they haven't picked pronouns and housing
        return self >= RegistrationStatus.KIND_CONFIRMED

    @property
    def is_failed(self) -> bool:
        return self == RegistrationStatus.FAILED


class OngoingRegistration:
    user_id: int
    status: RegistrationStatus
    provided_name: Optional[str]
    expected_kind: Optional[MemberKind]

    def __init__(self,
                 user_id: int,
                 status: RegistrationStatus,
                 provided_name: Optional[str] = None,
                 expected_kind: Optional[MemberKind] = None):
        self.user_id = user_id
        self.status = status
        self.provided_name = provided_name
        self.expected_kind = expected_kind
        self._validate_provided_registration()

    def _validate_provided_registration(self):
        match self.status:
            case (RegistrationStatus.UNREGISTERED | RegistrationStatus.STARTED):
                assert self.provided_name is None and self.expected_kind is None
            case (RegistrationStatus.NAME_ENTERED | RegistrationStatus.NAME_CONFIRMED):
                assert self.provided_name is not None and self.expected_kind is None
            case RegistrationStatus.KIND_CONFIRMED:
                assert self.provided_name is not None and self.expected_kind is not None
            # NOTE: we don't care what the name and kind are when registration fails
            # since failure can occur at any step
