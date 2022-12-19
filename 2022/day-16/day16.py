import re
import itertools
from typing import *

Valves = Dict[str, Any]
VisitedValves = FrozenSet[str]
Distances = Dict[str, Dict[str, int]]


def parse_input(contents: str) -> Valves:
    v = dict()
    for line in contents.splitlines():
        name, *connections = re.findall(r"[A-Z]{2}", line)
        rate = int(next(iter(re.findall(r"\d+", line))))
        v[name] = dict(to=frozenset(connections), rate=rate)
    return v


def compute_distances(valves: Valves) -> Distances:
    unreachable = float("+inf")
    dist = {
        i: {j: 1 if j in valve["to"] else unreachable for j in valves}
        for i, valve in valves.items()
    }
    for k, i, j in itertools.product(valves, valves, valves):
        dist[i][j] = min(dist[i][j], dist[i][k] + dist[k][j])
    return dist


def solve(
    valves: Valves, start: str, time_limit: int, distances: Distances
) -> Dict[VisitedValves, int]:

    solution: Dict[VisitedValves, int] = dict()
    valves_to_visit: Valves = {name: v for name, v in valves.items() if v["rate"] != 0}

    def _solve(
        current_valve: str,
        remaining_time: int,
        visited: VisitedValves,
        released_pressure: int,
    ):
        previous_pressure = solution.get(visited, 0)
        solution[visited] = max(previous_pressure, released_pressure)
        for valve_name, valve in valves_to_visit.items():
            new_remaining_time = (
                remaining_time - distances[current_valve][valve_name] - 1
            )
            if new_remaining_time <= 0 or valve_name in visited:
                continue
            _solve(
                valve_name,
                new_remaining_time,
                frozenset(itertools.chain(visited, (valve_name,))),
                released_pressure + new_remaining_time * valve["rate"],
            )

    _solve(start, time_limit, frozenset(), 0)
    return solution


with open("input.txt") as h:
    valves = parse_input(h.read())

dist = compute_distances(valves)

solution_part_one = solve(valves, "AA", 30, dist)
print(f"Part 1: {max(solution_part_one.values())}")

solution_part_two = solve(valves, "AA", 26, dist)

max_from_two_paths = max(
    my_pressure + elephant_pressure
    for my_visits, my_pressure in solution_part_two.items()
    for elephant_visits, elephant_pressure in solution_part_two.items()
    if set(elephant_visits).isdisjoint(set(my_visits))
)

print(f"Part 2: ", max_from_two_paths)
