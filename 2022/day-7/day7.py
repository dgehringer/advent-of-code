
import os
import dataclasses
from typing import *
from more_itertools import peekable


@dataclasses.dataclass
class Directory:
    name: str
    directories: List["Directory"]
    files: Dict[str, int]
    parent: Optional["Directory"]


def consume_output(input_iter: Iterable[str]):
    try:
        while not input_iter.peek().startswith('$'):
            yield next(input_iter)
    except StopIteration:
        return None


def empty_directory(name: str, parent: Optional[Directory] = None) -> Directory:
    return Directory(name, [], {}, parent)


def process_command(command: str, input_iter: Iterable[str], current_directory) -> NoReturn:
    assert command.startswith('$')
    _, command, *args = command.split(' ')
    if command == 'cd':
        folder, *_ = args
        if folder == '..':
            return current_directory.parent
        else:
            for d in current_directory.directories:
                if d.name == folder:
                    return d
            raise IOError(f'Could not find directory: {folder}')
    elif command == 'ls':
        files, dirs = dict(), list()
        for entry in consume_output(input_iter):
            if entry is None:  # in that case ls output is the end
                break
            elif entry.startswith('dir'):
                _, name = entry.split(' ')
                dirs.append(empty_directory(name, parent=current_directory))
            else:
                size, name = entry.split(' ')
                files[name] = int(size)
        current_directory.files = files
        current_directory.directories = dirs
        return current_directory


def read_fs(input_iter: str):
    fs = empty_directory('/')
    input_iter = peekable(iter(input_iter.splitlines()[1:]))
    current_directory = fs

    while input_iter:
        command = next(input_iter, None)
        if command:
            current_directory = process_command(command, input_iter, current_directory)
        else:
            break
    return fs


def size(fs: Directory):
    files_size = sum(fs.files.values())
    dir_sizes = map(size, fs.directories)
    return files_size + sum(dir_sizes)


def directories(fs: Directory):
    yield fs
    for d in fs.directories:
        yield from directories(d)


with open('input.txt') as h:
    fs = read_fs(h.read())

print('Part 1:', sum(size(d) for d in directories(fs) if size(d) < 100000))

disk_size = 70000000
needed_size = 30000000
cleaning_size = needed_size - disk_size + size(fs)

print("Part 2: ", min(size(d) for d in directories(fs) if size(d) >= cleaning_size))

