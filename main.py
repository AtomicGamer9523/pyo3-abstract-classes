from abstracto3 import Entity, GreeterEntity

class MyEntity(Entity):
    __last_id = 0
    def __init__(self):
        self._uuid = f'my-uuid-{MyEntity.__last_id}'
        MyEntity.__last_id += 1

    @property
    def uuid(self) -> str:
        return self._uuid

greeter = GreeterEntity()
my_entity = MyEntity()

greeter.greet('World')

print(greeter.uuid)
print(my_entity.uuid)
