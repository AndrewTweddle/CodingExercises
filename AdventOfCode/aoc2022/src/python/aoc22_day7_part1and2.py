root = {
    "name": "/",
    "entries": [],
    "size": None,
    "is_dir": True,
}
pwd_stack = [root]

def get_child_entry_with_name(dir, name):
    matches = [match for match in dir["entries"] if match["name"] == name]
    if len(matches) == 0:
        return None
    else:
        return matches[0]

with open("../../data/day7_input.txt", "r") as input_file:
    for line in input_file.readlines():
        line = line.rstrip()
        if line == "$ cd /":
            pwd_stack = [root]
        elif line == "$ cd ..":
            pwd_stack.pop()
        elif line.startswith("$ cd "):
            sub_dir_name = line[5:]
            pwd = pwd_stack[-1]
            sub_dir = get_child_entry_with_name(pwd, sub_dir_name)
            pwd_stack.append(sub_dir)
        elif line != "$ ls":
            is_dir = line.startswith("dir ")
            if is_dir:
                entry_name = line[4:]
                entry_size = None
            else:
                splits = line.split()
                size_str, entry_name = splits[0], splits[1]
                entry_size = int(size_str)

            # Only add the entry if it hasn't been visited before
            pwd = pwd_stack[-1]
            if get_child_entry_with_name(pwd, entry_name) is None:
                new_entry = {
                    "name": entry_name,
                    "entries": [],
                    "size": entry_size,
                    "is_dir": is_dir,
                }
                pwd["entries"].append(new_entry)

def calculate_dir_and_subdir_sizes(dir):
    if dir["size"] is None:
        total_size = 0
        for child in dir["entries"]:
            if child["is_dir"]:
                calculate_dir_and_subdir_sizes(child)
            total_size += child["size"]
        dir["size"] = total_size

calculate_dir_and_subdir_sizes(root)

def get_dir_and_all_sub_dirs(dir):
    all_sub_dirs = [dir]
    for child in dir["entries"]:
        if child["is_dir"]:
            all_sub_dirs += get_dir_and_all_sub_dirs(child)
    return all_sub_dirs

all_dirs = get_dir_and_all_sub_dirs(root)

# Solve part 1
part1_answer = sum([dir["size"] for dir in all_dirs if dir["size"] <= 100000])
print("part 1 answer: ", part1_answer)

# Solve part 2
actual_unused_space = 70000000 - root["size"]
space_to_free = max(0, 30000000 - actual_unused_space)
part2_answer = min([dir["size"] for dir in all_dirs if dir["size"] >= space_to_free])
print("part 2 answer: ", part2_answer)
