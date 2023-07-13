from enum import Enum

from sqlalchemy import String
from sqlalchemy.dialects.mysql import BIGINT
from sqlalchemy.orm import Mapped, mapped_column

from .base import DataModel


class VerifiedUserKind(Enum):
    """
    Represents the various kinds of `VerifiedUser`s.
    """

    UNDERGRAD = "UNDERGRAD"
    """
    An undergraduate student.
    """

    POSTGRAD = "POSTGRAD"
    """
    A postgraduate student.
    """

    MENTOR = "MENTOR"
    """
    A mentor.
    """

    SENIOR_MENTOR = "SENIOR_MENTOR"
    """
    A senior mentor.
    """

    HONORARY_MENTOR = "HONORARY_MENTOR"
    """
    An honorary mentor.
    """

    FACULTY = "FACULTY"
    """
    A member of faculty.
    """

    def __str__(self) -> str:
        return self.value

    def description(self) -> str:
        """
        Describes the `VerifiedUserKind` in a user-friendly, human-readable way.
        """
        # mfw no expression-oriented syntax, so I have to do this
        match self:
            case VerifiedUserKind.UNDERGRAD:
                return "first-year undergraduate student"
            case VerifiedUserKind.POSTGRAD:
                return "first-year postgraduate student"
            case VerifiedUserKind.MENTOR:
                return "mentor"
            case VerifiedUserKind.SENIOR_MENTOR:
                return "senior mentor"
            case VerifiedUserKind.HONORARY_MENTOR:
                return "honorary mentor"
            case VerifiedUserKind.FACULTY:
                return "member of faculty"


class VerifiedUser(DataModel):
    """
    Data about a user who is expected to join, along with their Discord user ID if
    they have completed registration.

    During registration, users are validated against a database of `VerifiedUser`s
    using their confirmed name and `VerifiedUserKind`. Those without a matching
    unregistered entry fail registration.
    """

    __tablename__ = "users"

    id: Mapped[int] = mapped_column(BIGINT(unsigned=True),
                                    autoincrement=True,
                                    primary_key=True)
    """
    The unique ID of the user in the database.
    """

    name: Mapped[str] = mapped_column(String(1024))
    """
    The user's full name, exactly as they applied to university with.
    """

    kind: Mapped[VerifiedUserKind]
    """
    What kind of user this is.
    """

    registered_user_id: Mapped[int | None] = mapped_column(BIGINT(unsigned=True))
    """
    The user's Discord user ID if they have registered.

    This is `None` if the user has not completed registration.
    """
