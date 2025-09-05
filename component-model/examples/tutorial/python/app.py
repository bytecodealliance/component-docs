from wit_world import exports

class Add(exports.Add):
    def add(self, x: int, y: int) -> int:
        return x + y
