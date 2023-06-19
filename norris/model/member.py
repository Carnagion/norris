from enum import Enum

from sqlalchemy import Enum as SqlEnum
from sqlalchemy import String
from sqlalchemy.orm import Mapped, mapped_column

from .base import DataModel


class MemberKind(Enum):
    UNDERGRAD = 0
    POSTGRAD = 1
    MENTOR = 2
    SENIOR_MENTOR = 3
    HONORARY_MENTOR = 4
    FACULTY = 5

    def __str__(self) -> str:
        # mfw no expression-oriented syntax, so I have to do this
        match self:
            case MemberKind.UNDERGRAD:
                return "first-year undergraduate student"
            case MemberKind.POSTGRAD:
                return "first-year postgraduate student"
            case MemberKind.MENTOR:
                return "mentor"
            case MemberKind.SENIOR_MENTOR:
                return "senior mentor"
            case MemberKind.HONORARY_MENTOR:
                return "honorary mentor"
            case MemberKind.FACULTY:
                return "member of faculty"


class VerifiedMember(DataModel):
    __tablename__ = "users"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(1024))
    kind: Mapped[MemberKind] = mapped_column(SqlEnum(MemberKind))
    registered_user_id: Mapped[int | None]
