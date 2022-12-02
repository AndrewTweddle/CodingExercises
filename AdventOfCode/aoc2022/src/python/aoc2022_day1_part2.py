input_file = open("../../data/day1_input.txt", "r")

max_sums = [0, 0, 0]
curr_sum = 0

for s in input_file.readlines():
    if len(s.strip()) == 0:
        if curr_sum > max_sums[0]:
            max_sums[0] = curr_sum
            max_sums.sort()
        curr_sum = 0
    else:
        curr_sum += int(s)

if curr_sum > max_sums[0]:
    max_sums[0] = curr_sum

top_3_inventories = sum(max_sums)

print("AOC 2022: day 1, part 2: {}".format(top_3_inventories))
