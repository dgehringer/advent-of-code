lines = readlines(open("input.txt"))

to_binary = Dict('B' => '1', 'F' => '0', 'L' => '0', 'R' => '1')

idlist =
    map(
        conv -> parse(Int, conv[1:7], base=2)*8+ parse(Int, conv[end-2:end], base=2),
        map(
            boarding_pass -> String(
                map(
                    character -> to_binary[character], 
                    boarding_pass)
                ), lines
        )
    )

max_seat = max(idlist...)

# the real number of seats in this aricraft
min_seat = min(idlist...)
num_seats = max_seat - min_seat
# now the seat with minimal id has index 1 in our array. the shift between the array and the ID' is min(idlist...)
# we add one seat which is our own
full_seats = falses(num_seats+1)
for id in idlist
    full_seats[id-min_seat+1] = true
end
# correct for the shift
my_seat = argmin(full_seats) + min_seat - 1
