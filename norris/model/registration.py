from dataclasses import dataclass
from enum import Enum

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


@dataclass()
class OngoingRegistration:
    user_id: int
    status: RegistrationStatus
    provided_name: str | None
    expected_kind: MemberKind | None

    def __init__(self,
                 user_id: int,
                 status: RegistrationStatus,
                 provided_name: str | None = None,
                 expected_kind: MemberKind | None = None) -> None:
        self.user_id = user_id
        self.status = status
        self.provided_name = provided_name
        self.expected_kind = expected_kind
        if not self.is_state_valid():
            raise RegistrationStateError(status, provided_name, expected_kind)

    def is_state_valid(self) -> bool:
        match self.status:
            case (RegistrationStatus.UNREGISTERED | RegistrationStatus.STARTED):
                return self.provided_name is None and self.expected_kind is None
            case (RegistrationStatus.NAME_ENTERED | RegistrationStatus.NAME_CONFIRMED):
                return self.provided_name is not None and self.expected_kind is None
            case (RegistrationStatus.KIND_CONFIRMED |
                  RegistrationStatus.PRONOUNS_PICKED |
                  RegistrationStatus.HOUSING_PICKED):
                return self.provided_name is not None and self.expected_kind is not None
            # NOTE: we don't care what the name and kind are when registration fails
            # since failure can occur at any step
            case _:
                return True


class RegistrationStateError(Exception):
    def __init__(self,
                 status: RegistrationStatus,
                 provided_name: str | None,
                 expected_kind: str | None) -> None:
        super().__init__(f"Name and kind provided are '{provided_name}' and "
                         f"{expected_kind}, but registration status is {status}")
