to_seats(c) = hcat(map(collect, split(c, "\n"))...)
content = read("input.txt", String)[1:end-1]
seats = to_seats(content)
(h,w) = size(seats)

# get the relavtive array coordinates, or directions
stencil = [(i,j) for i=-1:1, j=-1:1 if !(i == 0 && j == 0)]
in_bounds(i, j) = 1 <= i <= h && 1 <= j <= w

mutate(s) = [ rules(s[i,j],  occupied_around(i,j, s)) for i=1:h, j=1:w]
occupied(s) = count(e -> e =='#', s)

# helper function run the simulation
function run(seats)
    s = copy(seats)
    ns = mutate(s)
    while abs(occupied(ns) - occupied(s)) != 0
        s = ns
        ns = mutate(s)
    end
    occupied(ns)
end

# rules and occupancy fules for part one
function rules(seat, occ)
    (seat == 'L' && occ == 0) && return '#'
    (seat == '#' && occ >= 4) && return 'L'
    return seat
end
places_around(i, j, seats) = [CartesianIndex(i+di, j+dj) for (di, dj) in stencil if in_bounds(i+di, j+dj)]
occupied_around(i, j, seats) = count(e -> e == '#', seats[places_around(i, j, seats)])

run(seats)

# rules and occupancy rules for part two
function rules(seat, occ)
    (seat == 'L' && occ == 0) && return '#'
    (seat == '#' && occ >= 5) && return 'L'
    return seat
end

function occupied_around(i, j, seats)
    occupied = 0
    for direction in stencil
        (di, dj) = direction
        found_seat = false
        d = 1
        while in_bounds(i+di*d, j+dj*d) && !found_seat
            next = seats[i+di*d, j+dj*d]
            found_seat = next ∈ ('L', '#')
            d+=1
            if next == '#'
                occupied += 1
            end
        end
    end
    occupied
end

run(seats)


