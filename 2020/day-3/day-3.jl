data = map(collect, readlines(open("input.txt")))

tree = '#'

function slope(xslope, yslope)
    sx, sy  = length(data[1]), length(data)
    # we divide array length by y-slope to compute the number of vertical steps we have to make
    # furthermore i computeit 0-index based and add then one otherwise -- CONFUSION
    # I take floor do leave the last index if we overshoot
    sum(data[iy*yslope+1][((iy*xslope%sx)+1)] == tree for iy in 0:1:Int(floor(length(data) / yslope))-1)
end

println(slope(3,1))

slopes  = [[1,1], [3,1], [5,1], [7,1], [1,2]]

println(prod(map(sl -> slope(sl...), slopes)))


