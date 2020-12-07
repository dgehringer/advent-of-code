content= read(open("input.txt"), String)

sum(
    map(group-> length(
            union(
                map(person -> Base.Set(collect(person)), 
                        split(group, r"\n{1}")
                )...)
        ), 
        split(content, r"\n{2,}")
    )
)

sum(
    map(group-> length(
            intersect(
                map(person -> Base.Set(collect(person)), 
                        split(group, r"\n{1}")
                )...)
        ), 
        split(content, r"\n{2,}")
    )
)+1
