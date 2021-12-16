import io
import operator
import itertools
import functools
import collections.abc

def transcribe(s):
    return ''.join(map(lambda c: bin(int(c, 16))[2:].zfill(4), s))

def take(it, n):
    return ''.join(itertools.islice(it, n))

def to_dec(s: str):
    return int(s, 2)

def is_empty(it):
    peek = next(it, None)
    return (peek is None, it if peek is None else itertools.chain([peek], it))

def read_packet(stream):
    packet_version = to_dec(take(stream, 3))
    packet_type = to_dec(take(stream, 3))
    packet_parsers = {4: read_packet_literal}
    return packet_parsers.get(packet_type, read_packet_operator)(packet_version, packet_type, stream)

def read_packet_literal(version, ptype, stream):
    curr_bit = None
    buf = io.StringIO()
    while curr_bit is None or curr_bit.startswith('1'):
        curr_bit = take(stream, 5)
        buf.write(curr_bit[1:])
    return (version, ptype, to_dec(buf.getvalue()))

def read_packet_operator(version, ptype, stream):
    length_type = to_dec(take(stream, 1))
    packet_parsers = {0: read_packet_operator_zero, 1: read_packet_operator_one}
    return packet_parsers.get(length_type)(version, ptype, stream)

def read_packet_operator_zero(version, ptype, stream):
    packet_length = to_dec(take(stream, 15))
    substream = iter(take(stream, packet_length))
    empty = False
    packets = []
    while not empty:
        packets.append(read_packet(substream))
        empty, substream = is_empty(substream)
    return (version, ptype, packets)

def read_packet_operator_one(version, ptype, stream):
    num_packets = to_dec(take(stream, 11))
    return (version, ptype, [read_packet(stream) for _ in range(num_packets)])

def extract_version(packet):
    v, _, packtes = packet
    yield v
    if isinstance(packtes, collections.abc.Iterable):
        for packet_ in packtes:
            yield from extract_version(packet_)

def reduction(f, converter=None):
    converter = converter or (lambda x: x)
    return lambda p: converter(functools.reduce(f, p))

op_codes = {
    0: reduction(operator.add),
    1: reduction(operator.mul),
    2: reduction(min),
    4: lambda x: x,
    3: reduction(max),
    5: reduction(lambda x, y: x > y, int),
    6: reduction(lambda x, y: x < y, int),
    7: reduction(lambda x, y: x == y, int)
}

def evaluate_packet(packet):
    _, packet_type, packets = packet
    if packet_type != 4:
        packets = map(evaluate_packet, packets)
    return op_codes.get(packet_type)(packets)

content = transcribe(open('input.txt').read().strip())
packet = read_packet(iter(content))

print("PROBLEM 1:", sum(extract_version(packet)))
print("PROBLEM 2:", evaluate_packet(packet))