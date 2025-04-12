# python有点麻烦，模块和变量都是source.python，没有更细的分类了

# 变量
from enum import Enum

a: int = 1
b: str = "hello world!"
c: bool = True

# 常量
PI: float = 3.14

# 列表
arr: list = [1, 2, 3]

# 字典
d: dict = {"a": 1, "b": 2}


# 函数
def add(a: int, b: int) -> int:
    return a + b


# 函数调用
add(a=1, b=2)


# 类
class Person:
    def __init__(self, name: str, age: int) -> None:
        self.name = name
        self.age = age

    def greet(self) -> None:
        print(f"Hello, my name is {self.name} and I am {self.age} years old.")


# 类调用
person = Person(name="John", age=30)
person.age
person.greet()


# 枚举
class Color(Enum):
    RED = 1
    GREEN = 2
    BLUE = 3


color = Color
color.BLUE
