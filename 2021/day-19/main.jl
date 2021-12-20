using StaticArrays
using LinearAlgebra
using IterTools: chain
using Base.Iterators
using DataStructures: Accumulator, inc!

const Mat3{T} = SMatrix{3, 3, T, 9} where T
const Vec3{T} = SVector{3, T} where T

function parse_scanner(data)
    lines = split(data, "\n")
    scanner_id = parse(Int, match(r"(\d+)", lines[1]).captures[1])
    positions = Set{Vec3{Int}}(lines[2:end] .|> (l) -> Vec3(parse.(Int, split(l, ","))))
    scanner_id => positions
end

function first_vec()
    b = Vec3(0, 0, 1)
    chain((circshift(b, i) for i in 1:3), (-circshift(b,i) for i in 1:3))
end

second_vec(first) = (sign*circshift(first, shift) for sign in -1:2:1 for shift in -1:2:1)

function third_vec(first, second)
    m  = abs.(abs.(first) + abs.(second) .- 1)
    (sign * m for sign in -1:2:1)
end

isometries() = ( Mat3([a b c]) for a in first_vec() for b in second_vec(a) for c in third_vec(a, b))

rotations() = Base.Iterators.filter((iso) -> det(iso) > 0, isometries())

map_set(f, s::Set{T}) where T = Set{T}((f(v) for v in s))

function arrange_scanners(scanners, start=0)
    scanner_positions = [Vec3(0,0,0)]
    remaining_scanners = [id for id in keys(scanners) if id != start] # we start already with the first open
    beacons = scanners[start]
    rots = collect(rotations())
    while length(remaining_scanners) > 0
        scanner = popfirst!(remaining_scanners)
        scanner_fitted = false
        for 𝐑 in rots
            acc = Accumulator{Vec3{Int}, Int}()
            rotated = map_set((p) -> 𝐑*p, scanners[scanner])
            for beacon in beacons
                for rot in rotated
                    inc!(acc, rot - beacon)
                end
            end
            inverse = Dict(v => k for (k,v) in pairs(acc))
            count = maximum(first, inverse)
            vector = inverse[count]
            if count >= 12
                beacons = union(beacons, map_set((r)-> r - vector, rotated))
                push!(scanner_positions, vector)
                scanner_fitted = true
                break
            end
        end
        !scanner_fitted && append!(remaining_scanners, scanner)
    end
    scanner_positions, beacons
end

content = read(open("input.txt"), String)
scanners = Dict(split(content, "\n\n") .|> parse_scanner)

positions, beacons = arrange_scanners(scanners)
println("Part 1: $(length(beacons))")

manhattan_norm = max( (sum(abs.(p1-p2)) for p1 in positions for p2 in positions)...)
println("Part 2: $(manhattan_norm)")
