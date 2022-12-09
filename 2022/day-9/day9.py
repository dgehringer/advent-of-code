
from typing import *
from math import copysign
from functools import partial
from operator import add, sub

Instruction = Tuple[str, int]
Position = Tuple[int, int]


def parse_line(line: str) -> Instruction:
    direction, amount = line.split()
    return direction, int(amount)


def modify(op: Callable[[int, int], int], head: Position, tail: Position) -> Position:
    hx, hy = head
    tx, ty = tail
    return op(hx, tx), op(hy, ty)


diff = partial(modify, sub)
move = partial(modify, add)


def clamp(a: int) -> int:
    return int(copysign(1, a)) if a else 0


def simulate(inst: Iterable[Instruction], length: int = 2):
    rope = [(0, 0)] * length
    visited = {rope[-1]}
    directions = dict(L=(-1, 0), R=(1, 0), U=(0, 1), D=(0, -1))
    for direction, amount in inst:
        vec = directions.get(direction)
        for _ in range(amount):
            head, *tail = rope
            rope[0] = move(head, vec)
            for i in range(length-1):
                head, tail = rope[i], rope[i+1]
                dx, dy = diff(head, tail)
                if abs(dx) > 1 or abs(dy) > 1:  # should move tail?
                    # make sure move vector points along dx and dy
                    rope[i+1] = move(tail, (clamp(dx), clamp(dy)))
                    if i == length - 2:  # tail of the rope as index i-2
                        visited |= {tail}
    return visited


with open('input.txt') as h:
    instructions = list(map(parse_line, h.read().splitlines()))

print('Part 1:', len(simulate(instructions)) + 1)
print('Part 2:', len(simulate(instructions, length=10)) + 1)
