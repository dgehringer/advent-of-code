
import Base.Iterators
import IterTools: peekiter, subsets

function parse_pair(s)
    @assert popfirst!(s) == '['
    c = peek(s)
    first = isdigit(c) ? parse(Int, popfirst!(s)) : parse_pair(s)
    @assert popfirst!(s) == ','
    c = peek(s)
    second = isdigit(c) ? parse(Int, popfirst!(s)) : parse_pair(s)
    @assert popfirst!(s) == ']'
    return [first, second]
end

function split_num(n)
    (d, r) = divrem(n, 2)
    [d, d+r]
end

function split(node)
    node isa Integer && return node > 9 ? (split_num(node), true) : (node, false)
    (l ,r) = node
    (l, is_split) = split(l)
    is_split && return [l,r], true
    (r, is_split) = split(r)
    [l, r], is_split
end

function update(node, v, dir)
    v ≡ Nothing && return node
    node isa Integer && return node + v
    (l, r) = node
    dir == :left ? [update(l, v, dir), r] : [l, update(r, v, dir)]
end

function explode(node, depth=4)
    node isa Integer && return Nothing, node, Nothing, false
    # if it's not a integer we can be sure it's a pair
    (l,r) = node
    depth == 0 && return l, 0, r, true
    (lft, l, rgt, exploded) = explode(l, depth-1)
    exploded && return lft, [l, update(r, rgt, :left)], Nothing, true
    (lft, r, rgt, exploded) = explode(r, depth-1)
    exploded && return Nothing, [update(l, lft, :right), r], rgt, true
    Nothing, node, Nothing, false
end

function mag(node)
    node isa Integer && return node
    (l, r) = node
    3 * mag(l) + 2 * mag(r)
end

function simplify(l, r)
    node = (l, r)
    while true
        _, node, _, exploded = explode(node)
        exploded && continue
        node, splitted = split(node)
        !splitted && break
    end
    node
end

lines = readlines("input.txt")
pairs = lines .|> (l) -> parse_pair(Iterators.Stateful(l))
println(mag(reduce(simplify, pairs)))
println(maximum(map( (pp)-> mag(simplify(pp...)), subsets(pairs, 2))))
