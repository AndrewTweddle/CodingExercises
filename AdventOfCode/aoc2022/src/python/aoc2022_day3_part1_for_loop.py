with open("../../data/day3_input.txt", "r") as input_file:
    sum_of_priorities = 0
    for line in input_file.readlines():
        items = [ord(ch) for ch in line]
        midpoint = len(line) // 2
        left = items[:midpoint]
        right = items[midpoint:]
        item = set(left).intersection(set(right)).pop()
        sum_of_priorities += 1 + item - ord('a') if item >= ord('a') else 27 + item - ord('A')
    print("AOC 2022: day 3, part 1:", sum_of_priorities)
