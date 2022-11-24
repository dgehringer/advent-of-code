
splitlines(s) = split(s, "\n")

# read the data as a 2D char array
grid = hcat(map(Vector{Char}, splitlines(read("input", String))[1:end-1])...)

(lx, ly) = size(grid)  # determine grid size


# reformat it as a dictionary
grid = Dict((i, j) => grid[i, j] for i in 1:lx for j in 1:ly)

move_east(i, j) =  (i % lx + 1, j)
move_south(i, j) = (i, j % ly + 1)

function step(grid, c, mover)
    new_grid = copy(grid)
    moved = false
    mvc = 0
    for ((i, j), grid_char) in pairs(grid)
        ni, nj  = mover(i, j)
        if grid_char == c && grid[(ni, nj)] == '.'
            new_grid[(i, j)] = '.'
            new_grid[(ni, nj)] = c
            moved = true
            mvc += 1
        end
    end
    println(mvc)
    new_grid, moved
end

function simulate(grid, east, south)
    moved = true
    steps = 0
    while moved
        (grid, moved) = simulate_step(grid, '>', east, moved)
        (grid, moved) = simulate_step(grid, 'v', south, moved)
        steps += 1
        println(steps, " ", moved)
        steps > 520 && break
    end
    return steps
end

println(simulate(grid, move_east, move_south))