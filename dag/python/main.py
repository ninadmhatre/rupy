import abc


class _Node:
    val: int

    @abc.abstractmethod
    def run(self) -> int:
        pass



class Value(_Node):
    def __init__(self, val: int):
        self.val = val

    def run(self) -> int:
        return self.val

    def __add__(self, other) -> "Value":
        if isinstance(self, type(other)):
            return Value(self.run() + other.run())
        if isinstance(other, int):
            return Value(self.run() + other)

        raise ValueError(f"{other} must be Value or int type, found {type(other)}")

    def __mul__(self, other) -> "Value":
        if isinstance(self, type(other)):
            return Value(self.run() * other.run())
        if isinstance(other, int):
            return Value(self.run() * other)

        raise ValueError(f"{other} must be Value or int type, found {type(other)}")

    def __sub__(self, other) -> "Value":
        if isinstance(self, type(other)):
            return Value(self.run() - other.run())
        if isinstance(other, int):
            return Value(self.run() - other)
        raise ValueError(f"{other} must be Value or int type, found {type(other)}")

    def __pow__(self, power, modulo=None) -> "Value":
        if isinstance(self, type(power)):
            return Value(self.run() ** power.run())
        if isinstance(power, int):
            return Value(self.run() ** power)

        raise ValueError(f"{power} must be Value or int type, found {type(power)}")

    def __str__(self) -> str:
        return str(self.val)

if __name__ == '__main__':
    a = Value(10)
    b = Value(5)

    a_cube = a ** 3
    b_cube = b ** 3

    a_sqr = a ** 2
    b_sqr = b ** 2

    three_a_sqr_b = a_sqr * b * 3
    three_a_b_sqr = a * b_sqr * 3

    result = (a_cube - three_a_sqr_b) + (three_a_b_sqr - b_cube)

    print(f"({a} - {b})^3 = {result.run()}")
    assert result.run() == 125, "Something is wrong!"

