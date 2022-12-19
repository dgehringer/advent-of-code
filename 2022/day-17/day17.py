import functools
from operator import itemgetter
from typing import *

Point = Tuple[int, int]
Rock = Rocks = Set[Point]

with open("input.txt") as h:
    JetPattern = h.read().strip()

Shapes = (
    {(0, 0), (1, 0), (2, 0), (3, 0)},
    {(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)},
    {(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)},
    {(0, 0), (0, 1), (0, 2), (0, 3)},
    {(0, 0), (0, 1), (1, 0), (1, 1)},
)

get_y = itemgetter(1)


def can_move_tile(rocks: Rocks, p: Point) -> bool:
    x, y = p
    return 0 <= x < 7 and y > 0 and p not in rocks


def can_move_shape(rocks: Rocks, shape: Rock, p: Point) -> bool:
    x, y = p
    return all(can_move_tile(rocks, (x + dx, y + dy)) for dx, dy in shape)


def place_shape(rocks: Rocks, jet_index: int, shape_index: int, height: int):
    x, y = 2, height + 5
    shape = Shapes[shape_index]
    can_move_shape_to = functools.partial(can_move_shape, rocks, shape)

    while can_move_shape_to((x, y - 1)):
        y -= 1
        jet = JetPattern[jet_index]
        if jet == "<" and can_move_shape_to((x - 1, y)):
            x -= 1
        if jet == ">" and can_move_shape_to((x + 1, y)):
            x += 1
        jet_index = (jet_index + 1) % len(JetPattern)
    rock = {(x + dx, y + dy) for dx, dy in shape}
    rocks |= set(rock)
    shape_index = (shape_index + 1) % len(Shapes)
    return jet_index, shape_index, max(height, max(map(get_y, rock)))


def find_ground_tiles(
    rocks: Rocks, height: int, max_num_tiles: int = 30
) -> Optional[Rocks]:

    ground_tiles = set()

    def visit(p: Point, visited: Set[Point]):
        x, y = p
        if (
            not can_move_tile(rocks, (x, height + y))
            or (x, y) in visited
            or len(visited) > max_num_tiles
        ):
            return
        visited.add((x, y))
        for neighbor in ((x - 1, y), (x + 1, y), (x, y - 1)):
            visit(neighbor, visited)

    for x in range(7):
        visit((x, 0), ground_tiles)

    return frozenset(ground_tiles) if len(ground_tiles) <= max_num_tiles else None


def simulate(iterations: int):

    jet_index, shape_index, height, remaining_height = 0, 0, 0, 0
    rocks = set()
    iteration_cycles = dict()

    while iterations > 0:
        jet_index, shape_index, height = place_shape(
            rocks, jet_index, shape_index, height
        )
        iterations -= 1

        ground_tiles = find_ground_tiles(rocks, height)

        if ground_tiles is None:
            continue
        cycle = (jet_index, shape_index, ground_tiles)
        if cycle in iteration_cycles:
            previous_height, previous_iterations = iteration_cycles[cycle]
            remaining_height += (height - previous_height) * (
                iterations // (previous_iterations - iterations)
            )
            iterations = iterations % (previous_iterations - iterations)
        iteration_cycles[cycle] = (height, iterations)
    return height + remaining_height


print("Part 1: ", simulate(2022))
print("Part 1: ", simulate(1000000000000))
