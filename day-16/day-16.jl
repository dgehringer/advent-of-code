import Base.Iterators
contents = read(open("input.txt"), String)
function partial(f,a...)
        ( (b...) -> f(a...,b...) )
end
parse_int = partial(parse, Int)

(rules, my, nearby) = split(contents, "\n\n")

rule_regex = r"([\w\s]+):\s+(\d+)-(\d+)\s+or\s+(\d+)-(\d+)"
ranges(l)=collect(l[s]:l[s+1] for s in 1:2:length(l))

read_tickets(text, skip=1, backskip=0) = split(text, "\n")[skip+1:end-backskip] .|> l -> split(l, ",") .|> parse_int

nearby = read_tickets(nearby, 1, 1)
my = read_tickets(my)[1]

rules = split(rules, "\n") .|> l -> match(rule_regex, l).captures |> t -> (name=t[1], rules=t[2:end] .|> parse_int |> ranges)

all_rules = Iterators.flatten(rules .|> r -> r.rules)

sum(num for ticket in nearby for num in ticket if all(num ∉ r for r in all_rules))

logical_or(a,b) = a .| b

valid_ticket(ticket) = all(any(t in r for r in all_rules) for t in ticket)
valid_tickets = filter(valid_ticket, nearby)
valid_tickets = hcat(valid_tickets...)

candidates = eachrow(valid_tickets) .|>  col-> (rules .|> r -> r.rules .|> rule -> col .|> num -> num ∈ rule) .|> e -> all(reduce(logical_or, e))
candidates = hcat(candidates...)
ordering = Dict()
for _ in 1:size(candidates, 1)
    col = findfirst(eachcol(candidates) .|> sum .== 1)
    row = findfirst(candidates[:, col])
    candidates[row, :] .= false
    candidates[:, col] .= false
    ordering[rules[row].name] = col
end

prod(my[v] for (k,v) in ordering if startswith(k, "departure"))
