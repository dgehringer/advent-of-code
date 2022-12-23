
wrap(ind::Int, len::Int) = 1 + mod(ind - 1, len)

function simulate(message::Vector{Int}, decryption_key::Int, rounds::Int)
    message_length = length(message)
    indices = collect(1:message_length)
    
    for _ in 1:rounds
        for i in 1:message_length
            index = first(indexin(i, indices))
            deleteat!(indices, index)
            new_index = wrap(index + message[i] * decryption_key, message_length - 1)
            insert!(indices, new_index, i)
        end
    end
    zero_index = indexin(0, message) |> first |> ((ind) -> indexin(ind, indices)) |> first
    
    sum(message[indices[wrap(zero_index + offset, message_length)]] * decryption_key for offset in 1000:1000:3000)
end

message = readlines("input.txt") .|> ((l) -> parse(Int, l))

println("Part 1: $(simulate(message, 1, 1))")
println("Part 2: $(simulate(message, 811589153, 10))")
