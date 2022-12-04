with open("../../data/day3_input.txt", "r") as input_file:
    sum_of_priorities = 0
    elves = [[ord(ch) for ch in line] for line in input_file.readlines()]
    groups = [tuple(elves[i:i+3]) for i in range(0, len(elves), 3)]
    for (elf0, elf1, elf2) in groups:
        badges = [badge for badge in elf0 if badge in elf1 and badge in elf2]
        item = badges[0]
        sum_of_priorities += (1 + item - ord('a') if item >= ord('a') else 27 + item - ord('A'))
    print("AOC 2022: day 3, part 2:", sum_of_priorities)
