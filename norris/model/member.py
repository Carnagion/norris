from enum import Enum

from typing import Optional


class MemberKind(Enum):
    UNDERGRAD = 0
    POSTGRAD = 1
    MENTOR = 2
    SENIOR_MENTOR = 3
    HONORARY_MENTOR = 4
    FACULTY = 5

    def __str__(self):
        match self:  # mfw no expression-oriented syntax, so I have to do this
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


class VerifiedMember:
    name: str
    kind: MemberKind
    registered_user_id: Optional[int]

    def __init__(self,
                 name: str,
                 kind: MemberKind,
                 registered_user_id: Optional[int] = None):
        self.name = name
        self.kind = kind
        self.registered_user_id = registered_user_id
