import Base.Iterators: countfrom, takewhile
text = readlines(open("input.txt"))

timestamp = parse(Int, text[1])
buses = map(bus -> parse(Int, bus), filter(bus -> bus != "x", split(text[2], ",")))

departing(time, buses) = map(bus -> time % bus == 0, buses)
departing_bus(time, buses) = filter(bus -> time % bus == 0, buses)[1]

depart_time = collect(takewhile(time -> !any(departing(time, buses)), countfrom(timestamp)))[end] + 1

departing_bus(depart_time, buses) * (depart_time - timestamp)

# We cannot skip the "x" buses here
# we search for a number t where the buses = moduli have increasing remainders
buses = map(bus -> bus == "x" ? 'x' : parse(Int, bus) , split(text[2], ","))
delay = collect(length(buses)-1:-1:0)
# helper function to transpose arrays
T(array) = collect(zip(array...))

# filter the "x" buses from the list
(working_buses, workin_delays) = T(filter( ((b,_),) -> b != 'x', collect(zip(buses, delay))))

function find_xi(x, modul)
    # I had no better idea than a trial-error approach to get xi coefficients
    for xx in countfrom(1)
        xx * x % modul == 1 && return xx
    end
end

function solve(b, m)
    # this is a simplistic implementation of https://de.wikipedia.org/wiki/Chinesischer_Restsatz
    # the englisch version is much more helpful: https://en.wikipedia.org/wiki/Chinese_remainder_theorem
    # and: https://www.youtube.com/watch?v=zIFehsBHB8o
    # m are the moduli = the bus numbers
    # b consecutive ordering
    N = prod(m)
    Ni = N .// m
    # its faster to solve it probably in mod Ni % mi
    xsimp = Ni .% m
    # we search for the xi coefficients
    xi = map(a -> find_xi(a...), zip(xsimp, m))
    return sum(map(prod, zip(b, xi, Ni))) % N
end

# we get here the last number so we have to subtract the number of buses
solve(workin_delays, working_buses).num - length(buses) + 1
