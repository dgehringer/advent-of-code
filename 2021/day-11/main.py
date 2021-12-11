
import itertools
import numpy as np
from functools import reduce

def parse_file(fn):
    return np.array(list(map(lambda l: list(map(int, l.strip())), open(fn))))

def star(f):
    return lambda x: f(*x)

def transpose(l):
    return list(zip(*l))

def adjacent(grid: np.ndarray, i: int, j: int):
    coords = ((x+i, y+j) for x, y in itertools.product(range(-1, 2), range(-1, 2)))
    w, h = grid.shape
    return filter(star(lambda x, y: ((x,y) != (i,j) and (0 <= x < w and 0 <= y < h ))), coords)

def nditer(A: np.ndarray):
    return ((p, A[p]) for p in itertools.product(*map(range, A.shape)))

def step(grid : np.ndarray):
    grid += 1

    seen = np.zeros_like(grid, dtype=bool)
    def flash(g):
        def adj(i, j):
            return tuple(transpose(adjacent(g, i, j)))
        num_seen = np.sum(seen)

        for (i, j), energy in nditer(g):
            if not seen[i, j] and energy > 9:
                g[adj(i,j)] += 1
                seen[i,j] = True
        
        if num_seen != np.sum(seen):
            flash(g)

    flash(grid)
    num_flashes = np.sum(grid > 9)
    grid[grid > 9] = 0
    return grid, num_flashes
        
grid = parse_file('input.txt')

def simulate(p, _):
    gr, nf = p
    ggr, nnf = step(gr)
    return ggr, nf + nnf

print(reduce(simulate, range(100), (grid, 0))[-1])

def simulate(p, stepn):
    gr, nf = p
    ggr, nnf = step(gr)
    if len(ggr.flat) == nnf:
        raise StopIteration(stepn + 1)
    return ggr, nf + nnf

print(reduce(simulate, itertools.count(100), (grid, 0)))
