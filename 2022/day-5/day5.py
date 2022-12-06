import re
from operator import methodcaller as method


def parse_input(inp: str):
    stacks_map, instructions = map(method('splitlines'), inp.split('\n\n'))
    stacks_ids = [int(m)for m in re.findall(r'\d+', stacks_map[-1])]
    stacks = {sid: [] for sid in stacks_ids}
    create_regex = re.compile(r'(\s{3}|\[[A-Z]\])\s?')
    for line in reversed(stacks_map[:-1]):
        for stack_id, crate in zip(stacks_ids, create_regex.findall(line)):
            if crate.strip():
                stacks[stack_id].insert(0, crate[1])
    instruction_regex = re.compile(r'move\s(\d+)\sfrom\s(\d+)\sto\s(\d+)')
    return stacks, [tuple(map(int, instruction_regex.match(inst).groups())) for inst in instructions]


def part_one(stacks, instructions):
    for amount, src, dst in instructions:
        for _ in range(amount):
            crate = stacks[src].pop(0)
            stacks[dst].insert(0, crate)
    print(''.join(stacks[sid][0] for sid in sorted(stacks)))


def part_two(stacks, instructions):
    for amount, src, dst in instructions:
        crates = stacks[src][:amount]
        del stacks[src][:amount]
        for crate in reversed(crates):
            stacks[dst].insert(0, crate)
    print(''.join(stacks[sid][0] for sid in sorted(stacks)))


with open("input.txt") as h:
    contents = h.read()
    part_one(*parse_input(contents))
    part_two(*parse_input(contents))
