import functools
import re
import math
import itertools
import numpy as np

def read_bounds(filename):
    minx, maxx, miny, maxy = map(int, re.findall(r'[+-]?\d+', open(filename).read()))
    return ((minx, maxx), (miny, maxy))

def inverse_gauusian_sum(n):
    return np.amax(np.roots([1,-1,-2*n]))

def calc_max_y(b):
    # as x and y coordinates are indpedent we do not have to care about x. Moreover the max(y) is where at the last step we have dy = miny
    _, (miny, _) = b
    return int((miny +1)*miny/2)

b = read_bounds('input.txt')
print(calc_max_y(b))

def in_bounds(x, y, b):
    (minx, maxx), (miny, maxy) = b
    return (minx <= x <= maxx) and (miny <= y <= maxy)

def missed(y, b):
    _, (miny, maxy) = b
    return y == min(miny, maxy, y)

def trajectory(b, v0):
    x = y = 0
    dx, dy = v0
    while not in_bounds(x, y, b) and not missed(y, b):
        x += dx
        y += dy
        dy -= 1
        dx = dx if dx == 0 else dx -1
    return in_bounds(x, y, b)

def count_hits(b):
    (minx, maxx), (miny, _) = b
    vx_min = int(math.floor(inverse_gauusian_sum(minx)))
    velocities =  itertools.product(range(vx_min, maxx+1), range(miny, abs(miny)+1))
    trajectory_ = functools.partial(trajectory, b)
    return sum(map(trajectory_, velocities))

print(count_hits(b))

