
function read_input()
    content = read(open("input.txt"), String)
    (algo, image) = split(content, "\n\n")    
    algo, hcat(map(collect, split(image, "\n"))...)
end

flat(A) = [A...]

to_dec(arr) = foldl(((acc, d) -> acc*2 + (d == '#' ? 1 : 0)), arr; init=0)

function grow_image(image)
    (h, w) = size(image)
    grown = fill('.', (h+4, w+4))
    grown[3:(2+h), 3:(2+w)] = image
    (h,w), grown
end

function simulate(algo, image)
    (h, w), gr = grow_image(image)
    new = fill('.', (h+2, w+2))
    for dh in 1:(h+2)
        for dw in 1:(w+2)
            @inbounds new[dh, dw] = algo[to_dec(flat(gr[dh:(dh+2), dw:(dw+2)]))+1]
        end
    end
    new
end

(algo, img) = read_input()

advance(image, i) = simulate(algo, image)

final = foldl(advance, 1:50; init=img)

println("Part 1: $(count(==('#'), final))")