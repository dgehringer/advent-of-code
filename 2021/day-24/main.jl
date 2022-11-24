
data = read("input", String)

chunks = map((c) -> split(c, "\n"), split(data, "inp w\n")[2:end])

should_pop(block) = parse(Int, block[4][end-1:end]) == 26

parse_terms(block) = (parse(Int, split(block[5], " ")[end]), parse(Int, split(block[15], " ")[end])) 


function assign!(monads, i, j, values)
    mmax, mmin = monads
    (mmax[i], mmax[j], mmin[i], mmin[j]) = values
end

z = []
mmax = zeros(Int, 14)
mmin = zeros(Int, 14)
for (i, block) in enumerate(chunks) 
    (xa, ya) = parse_terms(block)

    if !should_pop(block)
        push!(z, (i, ya))
    else
        (j, ya) = pop!(z)
        diff = xa + ya
        assign_to!(values) = assign!((mmax, mmin), i, j, values) # specialize the function

        if diff < 0
            assign_to!((9 + diff, 9, 1, 1 - diff))
        elseif diff > 0
            assign_to!((9, 9 - diff, 1 + diff, 1))
        else
            assign_to!((9, 9, 1, 1))
        end
    end
end

println(join(map(repr, mmax)))
println(join(map(repr, mmin)))
