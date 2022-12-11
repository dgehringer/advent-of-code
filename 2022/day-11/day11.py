
import re
import functools
from operator import mul, add, itemgetter as item
from typing import *

Operation = Callable[[int], int]
Predicate = Callable[[int], bool]
Monkey = Tuple[int, List[int], Operation, Operation]


def find_first_int(a: str) -> int:
    return int(next(iter(re.findall(r'\d+', a))))


def make_op(a: Union[str, int], op: str, b: Union[str, int]) -> Operation:
    operation = {'+': add, '*': mul}.get(op)
    if isinstance(a, str) and isinstance(b, str):
        return lambda x: operation(x, x)
    elif isinstance(a, int) and isinstance(b, int):
        return lambda _: a + b
    else:
        n = a if isinstance(a, int) else b
        return lambda x: operation(x, n)


def try_parse(f: str) -> Union[str, int]:
    try:
        return int(f)
    except ValueError:
        return f


def make_test(n: int) -> Predicate:
    return lambda x: x % n == 0


def parse_monkey(monkey_data: str) -> Tuple[Monkey, int]:
    monkey, items, op, test, succ, fail, *_ = monkey_data.splitlines()
    monkey_id = find_first_int(monkey)
    items = list(map(int, re.findall(r'\d+', items)))
    a, op, b = re.match(r'.*new\s?=\s?(old|\d+)\s?(\*|\+)\s?(old|\d+)', op).groups()
    op = make_op(try_parse(a), op, try_parse(b))
    divisible_by = find_first_int(test)
    succ, fail = find_first_int(succ), find_first_int(fail)
    tester = make_test(divisible_by)
    return (monkey_id, items, op, lambda w: succ if tester(w) else fail), divisible_by


def parse_input(inp: str):
    divisors, monkeys = [], []
    for monkey, divisor in map(parse_monkey, inp.split('\n\n')):
        monkeys.append(monkey)
        divisors.append(divisor)
    return monkeys, functools.reduce(mul, divisors, 1)


def play(monkeys: List[Monkey], rounds: int = 20, cut_worry_by: int = 3, modulo: int = 0) -> int:
    get_items = item(1)

    def throw_to_monkey(monkey: int, thing: int) -> NoReturn:
        get_items(monkeys[monkey]).append(thing)

    inspects = {monkey_id: 0 for monkey_id, *_ in monkeys}
    for _ in range(rounds):
        for monkey in monkeys:
            monkey_id, items, op, next_monkey = monkey
            for worry in items:
                worry = op(worry) // cut_worry_by
                throw_to_monkey(next_monkey(worry), worry % modulo if modulo else worry)
            inspects[monkey_id] += len(items)
            items.clear()
    most, second_most, *_ = sorted(inspects.values(), reverse=True)
    return most * second_most


with open('input.txt') as h:
    monkeys, global_modulo = parse_input(h.read())
print('Part 1: ', play(monkeys))
print('Part 2: ', play(monkeys, 10000, cut_worry_by=1, modulo=global_modulo))
