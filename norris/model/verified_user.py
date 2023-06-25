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
                return "First Year Undergraduate Student"
            case VerifiedUserKind.POSTGRAD:
                return "First Year Postgraduate Student"
            case VerifiedUserKind.MENTOR:
                return "Mentor"
            case VerifiedUserKind.SENIOR_MENTOR:
                return "Senior Mentor"
            case VerifiedUserKind.HONORARY_MENTOR:
                return "Honorary Mentor"
            case VerifiedUserKind.FACULTY:
                return "Member of Faculty"


class VerifiedUser(DataModel):
    __tablename__ = "users"

    id: Mapped[int] = mapped_column(BIGINT(unsigned=True),
                                    autoincrement=True,
                                    primary_key=True)
    name: Mapped[str] = mapped_column(String(1024))
    kind: Mapped[VerifiedUserKind]
    registered_user_id: Mapped[int | None] = mapped_column(BIGINT(unsigned=True))
