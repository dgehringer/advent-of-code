
@enum Direction begin
    up
    down
    forward
end

function parse_command(s::String)
    (direction, amount) = match(r"(up|down|forward)\s+(\d+)", s).captures
    (eval ∘ Symbol)(direction), parse(Int, amount)
end

operations = Dict(
    up => (h, d, a) -> (h, d - a),
    down => (h, d, a) -> (h, d + a),
    forward => (h, d, a) -> (h + a, d)
)

move(d::Direction, a::Int, pos) = operations[d](pos..., a)
move(d::Tuple{Direction, Int}, pos) = move(d..., pos)


commands = readlines("input.txt") .|> parse_command 

final_position = foldr(move, reverse(commands); init=(0,0))
println("Problem 1: $(prod(final_position))")

operations = Dict(
    up => (h, d, a, x) -> (h, d, a - x),
    down => (h, d, a, x) -> (h, d, a + x),
    forward => (h, d, a, x) -> (h + x, d  + a*x,  a)
)

final_position = foldr(move, reverse(commands); init=(0,0,0))
println("Problem 2: $(prod(final_position[1:2]))")
