input_file = open("../../data/day1_input.txt", "r")

max_sum = 0
curr_sum = 0

for s in input_file.readlines():
    if len(s.strip()) == 0:
        if curr_sum > max_sum:
            max_sum = curr_sum
        curr_sum = 0
    else:
        curr_sum += int(s)

if curr_sum > max_sum:
    max_sum = curr_sum

print("AOC 2022: day 1, part 1: {}".format(max_sum))

input_file.close()
