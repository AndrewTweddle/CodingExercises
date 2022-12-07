def pos_of_nth_distinct_char(msg, n):
    for pos in range(n, len(msg)):
        if len(set(msg[pos - n: pos])) == n:
            return pos

with open("../../data/day6_input.txt", "r") as input_file:
    msg = input_file.readline().rstrip()
    print("AOC 2022: day 6, part 1:", pos_of_nth_distinct_char(msg, 4))
    print("AOC 2022: day 6, part 2:", pos_of_nth_distinct_char(msg, 14))
