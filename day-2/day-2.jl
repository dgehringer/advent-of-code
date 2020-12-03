lines = readlines(open("input.txt"))

line_regex = r"^\s*(?<atleast>\d+)-{1}(?<atmost>\d+)\s*(?<character>[a-z]{1})\s*\:{1}\s*(?<password>[a-z]+)\s*$"
# parse the atleast and atmost regex groups and check it against their occurences
println(
    sum(
        [lower <= size(findall(Regex(character), pass))[1] <= upper
            for (character,lower, upper, pass) in 
                map(m -> (
                    m.captures[3],
                    parse(Int, m.captures[1]),
                    parse(Int, m.captures[2]), m.captures[end]), 
                    map(l -> match(line_regex, l), lines)
                )
            ]
    )
)

# as only one of the of thecharacters at the positions are allowed to match, we can weite it as an XOR condition
println(
    sum(
        [⊻(map(index -> p[parse(Int, index)] == character[1], [first, second])...) 
            for (first, second, character, p) in 
                map(l -> match(line_regex, l).captures, lines)
        ]
    )
)

