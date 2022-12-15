
import re
from multiprocessing import Pool, cpu_count
from typing import *

Point = Beacon = Sensor = Tuple[int, int]
Grid = List[Tuple[Beacon, Sensor]]

def parse_input(inp: str) -> Grid:
    coord_regex = re.compile(r'[x|y]=([+-]?\d+)')
    def parse_line(l: str) -> Tuple[Sensor, Beacon]:
        srow, scol, brow, bcol = map(int, coord_regex.findall(l))
        return (srow, scol), (brow, bcol)

    return list(map(parse_line, inp.splitlines()))

def dist(a: Point, b: Point) -> int:
    ar, ac = a
    br, bc = b
    return abs(br - ar) + abs(bc - ac)


def intervals_for_row(grid: Grid, target: int = 2000000) -> List[Point]:
    free_ranges = set()
    for sensor, nearest_beacon in grid:
        nearest_distance = dist(sensor, nearest_beacon)
        srow, scol = sensor
        dcol = abs(scol - target)
        if dcol <= nearest_distance:
            drow = nearest_distance - dcol
            free_ranges.add((srow - drow, srow + drow))

    first_range, *remaining_ranges = sorted(free_ranges)

    merged_intervals: List[Point] = [first_range]
    for rem_row, rem_col in remaining_ranges:
        last_row, last_col = merged_intervals[-1]
        if rem_row > last_col:
            merged_intervals.append((rem_row, rem_col))
        else:
            merged_intervals[-1] = (last_row, max(last_col, rem_col))

    return merged_intervals

def chunks(rmin: int, rmax: int, chunk_size: Optional[int] = None) -> Iterable[Iterable[int]]:
    chunk_size = chunk_size or abs(rmax - rmin) // cpu_count()
    if abs(rmax - rmin) % chunk_size != 0:
        raise ValueError
    indices = list(range(rmin, rmax+1, chunk_size))
    return [range(*r) for r in zip(indices[:-1], indices[1:])]



def find_freq(grid: Grid, rows: Iterable[int], max_row: int) -> Optional[int]:
    for row in rows:
        intervals = intervals_for_row(grid, row)
        if len(intervals) > 1:
            _, col = intervals[0]
            return (col +1)* max_row + row
    return None


def part_two(grid: Grid, max_row: int = 4000000):
    result = None
    with Pool() as pool:
        inputs = ((grid, rows, max_row) for rows in chunks(0, max_row))
        for freq in pool.starmap(find_freq, inputs, chunksize=1):
            if freq is not None:
                result = freq
    print('Part 2:', result)


def part_one(grid: Grid, target: int = 2000000):
    merge_col, merge_row = next(iter(intervals_for_row(grid, target)))
    print('Part 1:',  merge_row - merge_col)

with open('input.txt') as h:
    contents = h.read()

board = parse_input(contents)
part_one(board)
part_two(board)
