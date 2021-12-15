
import DataStructures: PriorityQueue, enqueue!, dequeue!
parse_int(c) = parse(Int ,c)

parse_file(filename) = hcat(map(l -> map(parse_int, split(l, "")), readlines(filename))...)

pushq!(q, cost, index) = enqueue!(q, cost, index)
    
function popq!(q)
    (k,v) = peek(q)
    delete!(q, k)
    (k,v)
end

function adjacent(grid, i, j)
    candidates = [(i+di, j+dj) for (di,dj) in ((0,-1),(0,1),(-1,0),(1,0))]
    n, m = size(grid)
    in_bounds(i, j) = (1 <= i <= n && 1 <= j <=m)
    return filter((p)->in_bounds(p...), candidates)
end

function search(grid)
    n, m = size(grid)
    seen = Set([(1,1)])
    q = PriorityQueue{Tuple{Int, Int}, Int}()
    pushq!(q, (1,1), 0)
    while !isempty(q)
        ((i, j), cost) = popq!(q)
        (i == n && j == m) && return cost
        for p in adjacent(grid, i, j)
            if p ∉ seen
                q[p] = cost + grid[p...]
                push!(seen, p)
            end
        end
    end
end

grid = parse_file("input.txt")

search(grid) # ploblem 1
extend(r, grid) = hvcat(r, [mod1.(grid .+ (i + j), 9) for i=0:r - 1 for j=0:r - 1]...)
search(extend(5, grid)) # problem 2