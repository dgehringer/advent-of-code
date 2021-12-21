using Memoize
import IterTools: iterated

wrap(n; limit=100) = (n % limit == 0) ? limit : n % limit
roll((r, x)) = (sum(wrap.([x+1, x+2, x+3])), x+3)
dice = iterated(roll, (6,3))

function deterministic_die_play(p1, p2, s1=0, s2=0)
    rolls = 0
    for (die, r) in dice
        any(≥(999), (s1, s2)) && break
        p1 = wrap(p1 + die; limit=10)
        s1 += p1
        p1, p2 = p2, p1
        s1, s2 = s2, s1
        rolls = r
    end
    min(s1, s2) * rolls
end

possible_combinations_to_roll = [(3,1),(4,3),(5,6),(6,7),(7,6),(8,3),(9,1)]

@memoize function dirac_die_play(p1, p2, s1=0, s2=0)
    s2 ≥ 21 && return 0, 1
    num_won_1 = num_won_2 = 0
    for (die, combinations) in possible_combinations_to_roll
        new_pos = (die + p1 == 10) ? 10 : (die + p1) % 10         
        (sub_won_1, sub_won_2)  = dirac_die_play(p2, new_pos, s2, s1 + new_pos)
        num_won_1 += combinations*sub_won_1
        num_won_2 += combinations*sub_won_2
    end
    return num_won_2, num_won_1
end

println("Part 1: $(deterministic_die_play(4, 7))")
println("Part 2: $(max(dirac_die_play(4, 7)...))")