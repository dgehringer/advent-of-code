@enum OpCode acc jmp nop

opcodes = Dict(string(code) => code for code in instances(OpCode))
instruction_regex = Regex("($(join(keys(opcodes), "|")))\\s+([+-]?\\d+)")
operations = Dict(
    acc => (p, value) -> (p.accumulator += value; p.ptr +=1),
    nop => (p, value) -> (p.ptr += 1),
    jmp => (p, value) -> (p.ptr += value)
)

function load_program(lines)
    instructions = map(
        ((code, val),) -> (opcodes[code], parse(Int, val)), 
        map(
            line -> match(instruction_regex, line).captures, 
            lines
        )
    )
    execution_state = falses(length(instructions))
    Program(1, 0, instructions, execution_state)
end

mutable struct Program
    ptr::Int
    accumulator::Int
    instructions::Array{Tuple{OpCode, Int}}
    executed::Array{Bool}
    Program(ptr, acc, instructions, exec) = new(ptr, acc, instructions, exec)
    Program(content::String) = load_program(split(content, "\n"))
    Program(file::IOStream) = load_program(readlines(file))
end

function execute(program::Program)
    while program.ptr <= length(program.instructions)
        if program.executed[program.ptr]
            break
        end
        program.executed[program.ptr] = true
        (opcode, value) = program.instructions[program.ptr]
        operations[opcode](program, value)
    end
    program
end

p = Program(open("input.txt"))
execute(p).accumulator

function fix(program::Program)
    other = Dict(jmp => nop, nop => jmp)
    for visited in findall(e -> e == true, program.executed)[2:end]
        (opcode, value) = program.instructions[visited]
        if opcode in keys(other)
            program.instructions[visited] = (other[opcode], value)
            tmp = Program(1, 0, program.instructions, falses(length(program.instructions)))
            execute(tmp)
            if !tmp.executed[end]
                program.instructions[visited] = (opcode, value)
            else
                println("Error in line: $(visited)")
                return Program(1, 0, program.instructions, falses(length(program.instructions)))
            end
        end
    end
    
end

execute(fix(p)).accumulator



