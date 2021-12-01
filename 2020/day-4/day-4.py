
import re
from functools import lru_cache
from itertools import cycle
# part 1
fields = ["byr", "eyr", "iyr", "hgt", "hcl", "ecl", "pid", "cid"]
required = set(fields) - {"cid"}
# avoid typing it twiche
validator = re.compile(r"(%s)" % "|".join(list(required)))
passports = re.split(r"\n{2,}", open("input.txt").read())

def is_valid(passport):
    return set(validator.findall(passport))  == required
print(sum(map(is_valid, passports)))


def passport_tokens(passport):
    for token in re.split(r"\s+|\n+", passport):
        if token:
            yield token.split(":")


hex_color_regex = re.compile(r"#{1}[0-9a-f]{6}")
eye_color_regex = re.compile(r"amb|blu|brn|gry|grn|hzl|oth")
height_regex = re.compile(r"(?P<value>\d{2,3})(?P<unit>in|cm)")

# avoid recompiling same regexes
@lru_cache(maxsize=3)
def integer_regex(num):
    return re.compile(rf"\d{{{num}}}$")


def validate_height(value, unit):
    return  150 <= int(value) <= 193 if unit == "cm" else 59 <= int(value) <= 76


def matches(r, s):
    return r.match(s) is not None


def validate_fields(byr, iyr, eyr, hcl, ecl, pid, hgt, **kwargs):
    hgt_match  = height_regex.match(hgt)
    rules = ([
        matches(integer_regex(4), byr) and 1920 <= int(byr) <= 2002,
        matches(integer_regex(4), iyr) and 2010 <= int(iyr) <= 2020,
        matches(integer_regex(4), eyr) and 2020 <= int(eyr) <= 2030,
        matches(hex_color_regex, hcl),
        matches(eye_color_regex, ecl),
        matches(integer_regex(9), pid),
        matches(height_regex, hgt) and validate_height(**hgt_match.groupdict())
    ])
    return all(rules)


print(
    sum(
        map(
            lambda p: validate_fields(p=p,**dict(passport_tokens(p))),
            filter(is_valid, passports)
        )
    )
)

