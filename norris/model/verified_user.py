from enum import Enum

from sqlalchemy import String
from sqlalchemy.dialects.mysql import BIGINT
from sqlalchemy.orm import Mapped, mapped_column

from .base import DataModel


class VerifiedUserKind(Enum):
    UNDERGRAD = "undergrad"
    POSTGRAD = "postgrad"
    MENTOR = "mentor"
    SENIOR_MENTOR = "senior_mentor"
    HONORARY_MENTOR = "honorary_mentor"
    FACULTY = "faculty"

    def description(self) -> str:
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

    id: Mapped[int] = mapped_column(BIGINT(unsigned=True),
                                    autoincrement=True,
                                    primary_key=True)
    name: Mapped[str] = mapped_column(String(1024))
    kind: Mapped[VerifiedUserKind]
    registered_user_id: Mapped[int | None] = mapped_column(BIGINT(unsigned=True))
