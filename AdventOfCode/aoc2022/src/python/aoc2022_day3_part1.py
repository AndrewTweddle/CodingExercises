with open("../../data/day3_input.txt", "r") as input_file:
    item_lines_and_midpoints = [([ord(ch) for ch in line], len(line) // 2) for line in input_file.readlines()]
    compartment_pairs = [(items[:midpoint], items[midpoint:]) for items, midpoint in item_lines_and_midpoints]
    common_items = [set(left).intersection(set(right)).pop() for left, right in compartment_pairs]
    priorities = [1 + item - ord('a') if item >= ord('a') else 27 + item - ord('A') for item in common_items]
    print(f"AOC 2022: day 3, part 1: {sum(priorities)}")
