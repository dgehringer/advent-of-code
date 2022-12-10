
import dataclasses
import io
from typing import *
from functools import lru_cache, partial


@dataclasses.dataclass
class CPU:
    clock: int
    register: int


Instruction = Callable[[CPU], Iterable]
Program = Iterable[Instruction]


def noop(_: CPU):
    yield


def addx(v: int, cpu: CPU):
    yield from noop(cpu)
    cpu.register += v
    yield


def parse_input(inp: str) -> Program:
    @lru_cache(128)
    def get_command(l: str):
        if l.startswith('noop'):
            return noop
        elif l.startswith('addx'):
            _, v = l.split(' ')
            return partial(addx, int(v))
        else:
            raise ArithmeticError
    return map(get_command, inp.splitlines())


def run(program: Program, cpu: Optional[CPU] = None, offset: int = 20, interval: int = 40) -> Iterable[int]:
    cpu = cpu or CPU(1, 1)
    display = io.StringIO()
    for instruction in program:
        for _ in instruction(cpu):
            print_pixel(cpu, file=display)
            cpu.clock += 1
            if (cpu.clock - offset) % interval == 0:
                yield cpu.clock * cpu.register

    display = display.getvalue()
    for line in display.splitlines():
        print(line[-1:] + line[:-1])  # rotate line by one


def print_pixel(cpu: CPU, width: int = 40, file=None):
    print('#' if abs(cpu.clock % width - cpu.register) <= 1 else '.', end='', file=file)
    if cpu.clock % width == 0:
        print(file=file)


with open('input.txt') as h:
    inp = h.read()
print('Part 2:')
print('Part 1:', sum(run(parse_input(inp))))