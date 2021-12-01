content = readlines(open("input.txt"))
a = sort!(map(l -> parse(Int, l), content))
a = [0, a..., max(a...)+3]

counts = 
Dict(
    num => count(
        i -> a[i+1] - a[i] == num, 
        1:length(a)-1
    ) for num  in (1,3)
)
prod(values(counts))

combinations = zeros(Int, length(a))
combinations[1] = 1
# with cap we avoid summing over negative indices
cap(num, c) = num < c ? c : num
for i in 2:length(a)
    # simply sum over the previous ways
    combinations[i] = sum(combinations[j] for j in cap(i-3, 1):i if a[i] <= (a[j]+3))
end
combinations[end]


