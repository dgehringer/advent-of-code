
import re
from typing import *


def parse_inputs(inp: str) -> List[Tuple[Set[int], Set[int]]]:
    extractor = re.compile(r'(\d+)-(\d+),(\d+)-(\d+)')
    splitted = [map(int, extractor.match(l.strip()).groups()) for l in inp.splitlines()]
    return [(set(range(min1, max1+1)), set(range(min2, max2+1))) for (min1, max1, min2, max2) in splitted]


with open('input.txt', 'r') as h:
    ranges = parse_inputs(h.read())

print("Part1: ", sum(s1.issubset(s2) or s2.issubset(s1) for s1, s2 in ranges))
print("Part2: ", sum(s1.intersection(s2) != set() for s1, s2 in ranges))
