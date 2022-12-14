
import re
from typing import *
from math import copysign
from operator import itemgetter as item

Point = Tuple[int, int]
Rocks = Set[Point]
Board = Tuple[Rocks, Point, Point]
FinishingCondition = Callable[[Board, Point], bool]


def add(a: Point,  b: Point) -> Point:
    ar, ad = a
    br, bd = b
    return br + ar, bd + ad


def sub(a: Point, b: Point) -> Point:
    br, bd = b
    return add(a, (-br, -bd))


def clamp(a: int) -> int:
    return int(copysign(1, a))


def unit_vector(a: Point) -> Point:
    ar, ad = a
    return clamp(ar) if ar != 0 else 0, clamp(ad) if ad != 0 else 0


def make_line(start: Point, end: Point) -> Rocks:
    rocks = {start, end}
    p = start
    d = unit_vector(sub(end, start))
    while p != end:
        p = add(p, d)
        rocks.add(p)
    return rocks


def parse_input(inp: str, part_two=False) -> Board:
    point_regex = re.compile(r"(\d+),(\d+)")
    rocks: Rocks = set()

    for line in inp.splitlines():
        l = [tuple(map(int, m.groups())) for m in point_regex.finditer(line)]
        starts, ends = l[:-1], l[1:]
        for start, end in zip(starts, ends):
            rocks |= make_line(start, end)

    _, max_down = max(rocks, key=item(1))
    if part_two:
        max_down += 2
        # bottom line does not need to be 2 * max_down
        rocks |= make_line((500-max_down-5, max_down), (500 + max_down + 5, max_down))

    min_right, _ = min(rocks, key=item(0))
    max_right, _ = max(rocks, key=item(0))

    return rocks, (min_right, 0), (max_right, max_down)


def fell_into_abyss(board: Board, _: Point, new_pos: Optional[Point]) -> bool:
    if new_pos is None:
        return False
    _, _, (_, ld) = board
    _, pd = new_pos
    return pd > ld


def source_reached(_: Board, grain: Point, new_pos: Optional[Point]) -> bool:
    return grain == (500, 0) and new_pos is None


def simulate(board: Board, source: Point, finished: FinishingCondition) -> Rocks:
    rocks, _, (_, ld) = board
    sand = set()

    def can_move(gr: Point) -> Optional[Point]:
        directions = [(0, 1), (-1, 1), (1, 1)]
        for direction in directions:
            new_pos = add(gr, direction)
            if new_pos not in rocks and new_pos not in sand:
                return new_pos
        return None

    can_add_sand = True
    while can_add_sand:
        grain = source
        p = can_move(grain)
        can_add_sand = not finished(board, grain, p)
        while p is not None:
            grain = p
            p = can_move(grain)
            can_add_sand = not finished(board, grain, p)
            if not can_add_sand:
                break
        if can_add_sand:
            sand.add(grain)
        else:
            break
    return sand


with open('input.txt') as h:
     content = h.read()

source = (500, 0)

game_board = parse_input(content)
print('Part 1: ', len(simulate(game_board, source, fell_into_abyss)))
game_board = parse_input(content, part_two=True)
print('Part 2: ', len(simulate(game_board, source, source_reached))+1)