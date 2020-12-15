using ResumableFunctions
preamble = [0,20,7,16,1,18,15]

function next_numbers(preamble, turns)
    spoken = Dict(n => i - 1 for (i, n) in enumerate(preamble[1:end-1]))
    current = preamble[end]
    for turn in length(preamble)-1:turns-2
        next = (current ∈ keys(spoken)) ? turn - spoken[current] : 0
        spoken[current] = turn
        current = next
    end
    current
end

next_numbers(preamble, 2020)

next_numbers(preamble, 30000000)


