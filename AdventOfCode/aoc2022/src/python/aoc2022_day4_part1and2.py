import re

with open("../../data/day4_input.txt", "r") as input_file:
    part1 = 0
    part2 = 0
    for line in input_file.readlines():
        parts = re.split(r'[-,]', line.rstrip())
        l_start, l_end, r_start, r_end = [int(part) for part in parts]
        if (l_start <= r_start and l_end >= r_end) or (l_start >= r_start and l_end <= r_end):
            part1 += 1
        if l_start <= r_end and r_start <= l_end:
            part2 += 1
    print("AOC 2022: day 4, part 1:", part1)
    print("AOC 2022: day 4, part 2:", part2)
