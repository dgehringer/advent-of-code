
function regroup(state, coords...)
    (Bool(state == "on"), (parse(Int, c) for c in coords)...)
end

function parse_input(fname)
    lines = readlines(fname)
    
    lines .|> 
        ((l) -> match(r"(off|on)\s+x=([+-]?\d+)\.+([+-]?\d+),y=([+-]?\d+)\.+([+-]?\d+),z=([+-]?\d+)\.+([+-]?\d+)", l)) .|> 
        ((m) -> getfield(m, :captures)) .|> ((p) -> regroup(p...))
end

function intersecting_box(a::NTuple{6, Int}, b::NTuple{6, Int})
    maxx, maxy, maxz = (max(a[i], b[i]) for i in 1:2:5)
    minx, miny, minz = (min(a[i], b[i]) for i in 2:2:6)
    (minx - maxx >= 0 && miny - maxy >= 0 && minz - maxz >= 0) && return maxx, minx, maxy,  miny, maxz, minz
    nothing
end

function num_cubes(c::NTuple{6, Int})
    (x1, x2, y1, y2, z1, z2) = c
    (abs(x1-x2)+1) * (abs(y1-y2)+1) * (abs(z1-z2)+1)
end

function lighted_cubes(instructions)
    lights = 0
    counted_cubes = []
    for instruction in reverse(instructions)
        mode, cube = instruction[1], instruction[2:end]
        if mode
            dark_cubes = []
            for interb in map((cc) -> intersecting_box(cc, cube), counted_cubes)
                !isnothing(interb) && push!(dark_cubes, (true, interb...))
            end
            lights += num_cubes(cube)
            lights -= lighted_cubes(dark_cubes)
        end
        append!(counted_cubes, [cube])
    end
    lights
end


instructions = parse_input("input.txt")

fifty_box = (-50, 50, -50, 50, -50, 50)
part_one = [inst for inst in instructions if !isnothing(intersecting_box(inst[2:end], fifty_box))]
println(lighted_cubes(part_one))
println(lighted_cubes(instructions))
