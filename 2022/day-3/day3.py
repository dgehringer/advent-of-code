input = """vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw"""


from typing import *
from functools import reduce

def parse_input(inp: str) -> List[str]:
    return inp.splitlines()


def score(char: str) -> int:
    assert len(char) == 1
    return 1 + ord(char) - ord("a") if char.islower() else 27 + ord(char) - ord("A")


with open('input.txt') as h:
    backpacks = parse_input(h.read())


splitted_backpacks = [(set(b[: len(b) // 2]), set(b[len(b) // 2 :])) for b in backpacks]
common_elements = (next(iter(c1.intersection(c2))) for (c1, c2) in splitted_backpacks)

print("Part 1:", sum(map(score, common_elements)))


def chunks(lst: List[Any], n: int) -> Iterable[List[Any]]:
    for i in range(0, len(lst), n):
        yield lst[i:i + n]


def find_common(elf_group: List[str]) -> str:
    elf_group = map(set, elf_group)
    return next(iter(reduce(set.intersection, elf_group)))

grouped_backpacks = chunks(backpacks, 3)
common_elements = map(find_common, grouped_backpacks)

print("Part 2:", sum(map(score, common_elements)))