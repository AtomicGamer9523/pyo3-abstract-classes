import abc as _abc

class Entity(_abc.ABC):
    """Base class for all entities."""

    @_abc.abstractproperty
    def uuid(self) -> str:
        """Unique identifier for this entity."""
        ...

class GreeterEntity(Entity):
    """A simple entity I created to prove the concept."""

    def __init__(self):
        """Initializes a new instance of the DotEntity class."""
        ...
    
    def greet(self, name: str) -> str:
        """Greets the given name."""
        ...

    @property
    def uuid(self) -> str:
        """Unique identifier for this entity."""
        ...
