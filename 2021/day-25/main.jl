
splitlines(s) = split(s, "\n")

# read the data as a 2D char array
grid = hcat(map(Vector{Char}, splitlines(read("input", String))[1:end-1])...)

(lx, ly) = size(grid)  # determine grid size

# reformat it as a dictionary
grid = Dict((i, j) => grid[i, j] for i in 1:lx for j in 1:ly)

move_east(i, j) =  (i % lx + 1, j)
move_south(i, j) = (i, j % ly + 1)

function step(grid, c, mover, moved)
    new_grid = copy(grid)
    moved = false
    for ((i, j), grid_char) in pairs(grid)
        ni, nj  = mover(i, j)
        if grid_char == c && grid[(ni, nj)] == '.'
            new_grid[(i, j)] = '.'
            new_grid[(ni, nj)] = c
            moved = true
        end
    end
    new_grid, moved
end

function simulate(grid, east, south)
    moved = true
    steps = 0

    while moved
        (grid, moved) = step(grid, '>', east, moved)
        (grid, moved) = step(grid, 'v', south, moved)
        steps += 1
    end
    return steps
end

println(simulate(grid, move_east, move_south))