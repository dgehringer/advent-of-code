import Base.Iterators: Stateful, Take
import DataStructures: CircularBuffer

content = readlines(open("input.txt"))

content = readlines(open("input.txt"))
stream = Stateful(map(l -> parse(Int, l), content))
preamble_length = 25

function sums_to(preamble::CircularBuffer{Int}, num::Int)
    # we could write this as a single line list comprehensions
    # any([ 40-n == nn for (i, n) in enumerate(preamble) for nn in preamble[i+1:end]])
    # but there we cannot early-exit from the loop, thus we write it more verbose
    for (i, n) in enumerate(preamble)
        for nn in preamble[i+1:end]
            num - n == nn && return true
        end
    end
    false
end


function check_stream(stream, preamble_length::Int)
    preamble = CircularBuffer{Int}(preamble_length)
    # initialize preamble
    for number in Take(stream, preamble_length)
        push!(preamble, number)
    end
    for number in stream
        #println("$(number): ", map(rule-> rule(number), rules))
        !sums_to(preamble, number) && return number
        # println(preamble)
        push!(preamble, number)
        
    end
    nothing
end

number = check_stream(stream, preamble_length)

function check_stream_range(stream, num)
    for i=1:length(stream), j=2:length(stream)
        if sum(stream[i:j]) == num
            return num, sum(stream[i:j]), min(stream[i:j]...) + max(stream[i:j]...)
        end
    end
end

content = readlines(open("input.txt"))
stream = map(l -> parse(Int, l), content)
preamble_length = 25
check_stream_range(stream, number)

