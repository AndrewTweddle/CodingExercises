with open("../../data/day3_input.txt", "r") as input_file:
    elves = [[ord(ch) for ch in line] for line in input_file.readlines()]
    groups = [tuple(elves[i:i+3]) for i in range(0, len(elves), 3)]
    badges = [[badge for badge in elf0 if badge in elf1 and badge in elf2][0] for (elf0, elf1, elf2) in groups]
    priorities = [(1 + item - ord('a') if item >= ord('a') else 27 + item - ord('A')) for item in badges]
    print("AOC 2022: day 3, part 2:", sum(priorities))
