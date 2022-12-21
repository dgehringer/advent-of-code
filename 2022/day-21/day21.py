import re
from operator import sub, mul, truediv, add
from typing import *

Entry = Union[int, Tuple[str, int, int]]
Instructions = Dict[str, Entry]

Operations = {'+': add, '-': sub, '/': truediv, '*': mul}
InvOperations = {'-': add, '+': sub, '*': truediv, '/': mul}


def parse_line(line: str) -> Tuple[str, Entry]:
    name, action = line.split(": ")
    if re.match(r"-?\d+", action):
        action = int(action)
    else:
        first, op, second = re.match(r"(\w{4})\s*([+\-*\/])\s*(\w{4})", action).groups()
        action = (op, first, second)
    return name, action


def is_operation(e: Entry) -> bool:
    return not isinstance(e, int)


def solve(name: str, instructions: Instructions) -> int:
    operation = instructions.get(name)
    if is_operation(operation):
        op, arg1, arg2 = operation
        return Operations.get(op)(solve(arg1, instructions), solve(arg2, instructions))
    else:
        return operation


def contains_expression(what: str, name: str, instructions: Instructions) -> bool:
    if what == name:
        return True
    op = instructions.get(name)
    if is_operation(op):
        _, left, right = op
        return contains_expression(what, left, instructions) or contains_expression(what, right, instructions)
    else:
        return False


# solves the value of "what" such that expr "name" == "eq"
def solve_for(what: str, name: str, eq: int, instructions: Instructions):
    operation = instructions.get(name)
    if is_operation(operation):
        op, left, right = operation
        solve_for_left = contains_expression(what, left, instructions)
        inv_op = InvOperations.get(op)
        to_solve, value = (left, solve(right, instructions)) if solve_for_left else (right, solve(left, instructions))
        if op in "+*":
            new_eq = inv_op(eq, value)
        elif op in "-/":
            # in case of l - r = eq two out-comes are possible 1.) l = eq + r 2.) r = l - eq
            # in case of l / r = eq two out-comes are possible 1.) l = eq * r 2.) r = l / eq
            new_eq = inv_op(eq, value) if solve_for_left else Operations.get(op)(value, eq)
        else:
            raise ValueError
        if to_solve == what:
            return new_eq
        else:
            return solve_for(what, to_solve, new_eq, instructions)


with open("input.txt") as h:
    inst: Instructions = dict(map(parse_line, h.read().splitlines()))

print("Part 1: ", int(solve("root", inst)))

_, *args = inst.get("root")
inst["root"] = ["-"] + args
print("Part 2:", int(solve_for("humn", "root", 0, inst)))
