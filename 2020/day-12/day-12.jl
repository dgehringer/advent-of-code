R(α) = [cos(α) -sin(α); sin(α) cos(α) ]

manhattan(r) = sum(map(abs, r))

mutable struct Ship
    position
    orientation
    Ship() = new([0;0], [1; 0])
    Ship(position, orientation) = new(position, orientation)
end

commands = Dict(
    'N' => (s, d) -> s.position += d*[0,1],
    'S' => (s, d) -> s.position += d*[0,-1],
    'E' => (s, d) -> s.position += d*[1,0],
    'W' => (s, d) -> s.position += d*[-1,0],
    'F' => (s, d) -> s.position += d*s.orientation,
    'L' => (s, d) -> s.orientation = R(deg2rad(d)) * s.orientation,
    'R' => (s, d) -> s.orientation = R(deg2rad(-d)) * s.orientation
)
command_regex = Regex("([$(join(keys(commands), ""))])(\\d+)")
execute(comm, ship) = foreach( ((ins, dist), ) -> commands[ins](ship, dist), comm)

# parse commands into a tuple of characters and integers
list = map(
    m -> (m[1][1], parse(Int, m[2])),
    map(
        l -> match(command_regex, l).captures, 
        readlines(open("input.txt"))
    )
)
ship = Ship()

execute(list, ship)

manhattan(ship.position)

# all what changes between part 1 and 2 is that the ship does not move with respect to the unit vector but
# rather a general vector. In part one we used a unit vector to define orientation, here "roatation with respect to waypoint" means
# just rotating the general vector.
# As the orientation is defined by ship.orientation, so we just have to redefine E,W,N and S
commands['N'] = (s, d) -> s.orientation += d*[0,1]
commands['S'] = (s, d) -> s.orientation += d*[0,-1]
commands['E'] = (s, d) -> s.orientation += d*[1,0]
commands['W'] = (s, d) -> s.orientation += d*[-1,0]

ship = Ship([0; 0], [10; 1])

execute(list, ship)

ship.position

manhattan(ship.position)
