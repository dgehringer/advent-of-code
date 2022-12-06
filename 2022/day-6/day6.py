
def find_sequence(inp: str, message=4) -> int:
    for i in range(0, len(inp)-message):
        subset = inp[i:i+message]
        if len(set(subset)) == message:
            return i + message


with open('input.txt', 'r') as h:
    m = h.read()

print(find_sequence(m))
print(find_sequence(m, 14))
