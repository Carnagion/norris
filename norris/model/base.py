from sqlalchemy.orm import DeclarativeBase


class DataModel(DeclarativeBase):
    """
    Base class for all registration-related data models.
    """

    # NOTE: this is here to stop pdoc3 from including the superclass constructor docs in
    # every other class that inherits this, which is very annoying
    def __init__(self, **kwargs) -> None:
        pass
