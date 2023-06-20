from enum import Enum

from sqlalchemy import Enum as SqlEnum
from sqlalchemy import String
from sqlalchemy.orm import Mapped, mapped_column

from .base import DataModel


class VerifiedUserKind(Enum):
    UNDERGRAD = 0
    POSTGRAD = 1
    MENTOR = 2
    SENIOR_MENTOR = 3
    HONORARY_MENTOR = 4
    FACULTY = 5

    def __str__(self) -> str:
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
    __tablename__ = "users"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(1024))
    kind: Mapped[VerifiedUserKind] = mapped_column(SqlEnum(VerifiedUserKind))
    registered_user_id: Mapped[int | None]
