
from typing import *
from collections import deque

Grid = List[List[int]]
Point = Tuple[int, int]


def parse_input(inp: str) -> Tuple[Grid, Point, Point]:
    grid = []
    start, end = None, None
    for i, line in enumerate(inp.splitlines()):
        grid.append([])
        for j, char in enumerate(line):
            if char == 'S':
                start = (i, j)
                char = 'a'
            elif char == 'E':
                end = (i, j)
                char = 'z'
            grid[i].append(ord(char) - ord('a') + 1)

    return grid, start, end


def add(a: Point, b: Point) -> Point:
    ay, ax = a
    by, bx = b
    return ay + by, ax + bx


def find_path(grid: Grid, start: Point, end: Point) -> Optional[int]:
    ny, nx = (len(grid), len(grid[0]))
    seen: Set[Point] = set()
    q: Deque[Tuple[Point, int]] = deque([(start, 0)])
    costs = dict()
    while q:
        loc, cost = q.popleft()
        if loc == end:
            return cost
        if loc in seen:
            continue
        else:
            seen |= {loc}
        for d in [(-1, 0), (0, 1), (1, 0), (0, -1)]:
            py, px = add(loc, d)
            if 0 <= py < ny and 0 <= px < nx:
                y, x = loc
                if grid[py][px] <= 1 + grid[y][x]:
                    q.append(((py, px), cost + 1))
                    costs[(px, py)] = cost + 1


def minimal_path(grid: Grid, end: Point) -> Iterable[int]:
    for i, row in enumerate(grid):
        for j, elevation in enumerate(row):
            if elevation == 1:
                d = find_path(grid, (i, j), end)
                if d is not None:
                    yield d


with open('input.txt') as h:
    content = h.read()

landscape, s, e = parse_input(content)
print('Part 1: ', find_path(landscape, s, e))
print('Part 2: ', min(minimal_path(landscape, e)))
