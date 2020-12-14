using ResumableFunctions
import IterTools: product

function apply_bit!(number::Ref{UInt64}, index::Int, char::Char)
    char == 'X' && return
    value = parse(Int64, char)
    number[] = value == 0 ? number[] & ~(2^(index-1)) : number[] | 2^(index-1)
end

apply_mask(num::UInt64, mask::Array{Char,1}) = begin
    number = Ref{UInt64}(num)
    foreach(l -> apply_bit!(number, l...), enumerate(reverse(mask)))
    number[]
end

mask_regex = r"mask\s+=\s+([01X]{36})$"
mem_regex = r"mem\[(\d+)]\s+=\s+(\d+)"

instructions = readlines(open("input.txt"))

mutable struct Program
    mem::Dict{Int64, UInt64}
    mask::Array{Char, 1}
    Program() = new(Dict{Int64, Int64}(), collect("X"^36))
end

function run_program_part_one(p::Program, ins)
    for instruction in ins
        if startswith(instruction, "mask")
            p.mask = collect(match(mask_regex, instruction).captures[1])
        elseif startswith(instruction, "mem")
            pos, value = map(l -> parse(UInt64, l), match(mem_regex, instruction).captures)
            p.mem[pos] = apply_mask(value, p.mask)
        end
    end
    p
end



println(
    sum(
        values(
            run_program_part_one(
                Program(), 
                instructions).mem
        )
    )
)

function apply_mask_part_two!(mask::Array{Char,1}, address, dst::Array{Char,1})
    for (i, bit) in enumerate(reverse(mask))
        cb = (address >> (i - 1)) & 1
        # if the mask bit is zero we use the bit from the addres, otherwise we use the mask bit
        bit == '0' ? dst[i] = Char('0' + cb) : dst[i] = bit
    end
    reverse!(dst)
end

@resumable function addresses(mask::Array{Char, 1}, len = 0)
    num = zero(UInt64)
    # for each bit in the mask initialize it, set X to one in the beginning
    for (index, val) in enumerate(reverse(mask))
        val == '0' ? num &= ~(2^(index-1)) : num |= 2^(index-1)
    end
    indices = findall(reverse(mask) .== 'X')
    # wer create a list of all combinations
    combinations = Iterators.product([0:1 for _ in 1:length(indices)]...)
    for combination in combinations
        # we zip ith index and its correspondig bit value and store it on num
        for (index, val) in zip(indices, combination)
            val == 0 ? num &= ~(2^(index-1)) : num |= 2^(index-1)
        end
        @yield num
    end
end

function run_program_part_two(p::Program, ins)
    tmp_mask = copy(p.mask)
    for instruction in ins
        if startswith(instruction, "mask")
            # we set the mask if we find it
            p.mask = collect(match(mask_regex, instruction).captures[1])
        elseif startswith(instruction, "mem")
            pos, value = map(l -> parse(UInt64, l), match(mem_regex, instruction).captures)
            # new we generate the bitmask used to generate the addresses
            apply_mask_part_two!(p.mask, pos, tmp_mask)
            # do the batch write
            for address in addresses(tmp_mask)
                p.mem[address] = value
            end
        end
    end
    p
end

println(
    sum(
        values(
            run_program_part_two(
                Program(), 
                instructions).mem
        )
    )
)
