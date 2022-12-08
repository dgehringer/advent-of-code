
import numpy as np
from typing import *


def parse_input(inp: str) -> np.ndarray:
    return np.array([list(map(int, line)) for line in inp.splitlines()])


def get_directions(i: int, j: int, grid: np.ndarray) -> Tuple[np.ndarray, np.ndarray, np.ndarray, np.ndarray]:
    return grid[i, :j][::-1], grid[i, j+1:], grid[:i, j][::-1], grid[i+1:, j]


def find_visible(grid: np.ndarray) -> Iterable[Tuple[int, int, int]]:
    for (i, j), height in np.ndenumerate(grid):
        if any(np.all(direction < height) for direction in get_directions(i, j, grid)):
            yield i, j, height


def axis_score(height: int, ax: np.ndarray) -> int:
    if not ax.size:
        return 0
    score = 1
    for other_height in ax[:-1]:
        if height > other_height:
            score += 1
        else:
            break
    return score


def compute_view_score(grid: np.ndarray, visible: Iterable[Tuple[int, int, int]]) -> Iterable[int]:
    for i, j, height in visible:
        dirs = get_directions(i, j, grid)
        axis_scores = map(lambda arr: axis_score(height, arr), dirs)
        yield np.prod(list(axis_scores))


with open('input.txt') as h:
    board = parse_input(h.read())

print('Part 1:', sum(1 for _ in find_visible(board)))
print('Part 2:', max(compute_view_score(board, find_visible(board))))
