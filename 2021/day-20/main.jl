
function read_input()
    content = read(open("input.txt"), String)
    (algo, image) = split(content, "\n\n")    
    algo, hcat(map(collect, split(image, "\n"))...)
end

flat(A) = [A...]

to_dec(arr) = foldl(((acc, d) -> acc*2 + (d == '#' ? 1 : 0)), arr; init=0)

flip(c) = c == '#' ? '.' : '#'

function grow_image(image, bg)
    (h, w) = size(image)
    grown = fill(bg, (h+4, w+4))
    grown[3:(2+h), 3:(2+w)] = image
    (h,w), grown
end

function simulate(algo, image, bg='.', flipbg=false)
    (h, w), gr = grow_image(image, bg)
    (flipbg ? flip(bg) : bg, [algo[to_dec(flat(gr[dh:(dh+2), dw:(dw+2)]))+1] for dh in 1:(h+2),  dw in 1:(w+2)])
end

(algo, img) = read_input()

flipbg = algo[1] == '#' && algo[end] == '.'

function advance(d, i)
    (bg, image) = d
    simulate(algo, image, bg, flipbg)
end

(_, final) = foldl(advance, 1:2; init=('.', img))
println("Part 1: $(count(==('#'), final))")

(_, final) = foldl(advance, 1:50; init=('.', img))
println("Part 2: $(count(==('#'), final))")