import json
from functools import partial, cmp_to_key
from typing import *

Packet = List[Union[List[int], int]]
PacketPair = Tuple[Packet, Packet]


def parse_input(inp: str) -> Iterable[PacketPair]:
    for packet_pair in inp.split('\n\n'):
        first, second = map(json.loads, packet_pair.splitlines())
        yield first, second


def both_instance(left: Union[Packet, int], right: Union[Packet, int], what: Type) -> bool:
    return isinstance(left, what) and isinstance(right, what)


def compare(a: int, b: int) -> int:
    return 0 if a == b else (1 if a < b else -1)


def compare_packet(left: Union[Packet, int], right: Union[Packet, int]) -> int:
    both_are = partial(both_instance, left, right)
    if both_are(int):
        return compare(left, right)
    elif both_are(list):
        llen, rlen = len(left), len(right)
        for i in range(min(llen, rlen)):
            ordering = compare_packet(left[i], right[i])
            if ordering:
                return ordering
        return compare(llen, rlen)
    else:
        left, right = ([left], right) if isinstance(left, int) else (left, [right])
        return compare_packet(left, right)


with open('input.txt') as h:
    contents = h.read()
packets = list(parse_input(contents))
print('Part 1: ', sum(i+1 for i, p in enumerate(packets) if compare_packet(*p) == 1))

dividers = [[[2]], [[6]]]
all_packets = [packet for pair in packets for packet in pair] + dividers
sorted_packages = sorted(all_packets, key=cmp_to_key(compare_packet), reverse=True)
first_div, second_div = dividers
print('Part 2:', (sorted_packages.index(first_div) + 1) * (sorted_packages.index(second_div) + 1))
