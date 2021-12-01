lines = readlines(open("input.txt"))

bag_regex = r"(\d+)?\s*(\w+)\s+(\w+)\s+bags?"
function parse_contents(rhs)
    occursin("no other",  rhs) && return 0
    Dict(
        map(
            ((num,a,b),) -> (a,b) => parse(Int, num), 
            map(
                b-> match(bag_regex, b).captures, split(rhs, ",")
            )
        )
    )
end

data = Dict(
    map( 
        ((lhs, rhs),) -> (
            Tuple(match(bag_regex, lhs).captures[end-1:end]),
            parse_contents(rhs)
        ), 
        map(
            line -> split(line, "contain"), lines)
        )
)

needle = ("shiny", "gold")
function can_hold_bag(mapping)
    mapping == 0 && return false
    needle ∈ keys(mapping) && return true
    return any(can_hold_bag(data[subbag]) for subbag ∈ keys(mapping))
end

sum(map(can_hold_bag, values(data)))

function bags_inside(needle)
    value = data[needle]
    value isa Number && return value
    sum(bag_count*bags_inside(bag)+bag_count for (bag, bag_count) ∈ value)
end

bags_inside(needle)


