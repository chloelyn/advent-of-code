import os

TOTAL_FS_SIZE = 70000000
NECESSARY_UNUSED = 30000000


def find(fs, name):
    for el in fs["entries"]:
        if el["name"] == name:
            return el
    assert False


def calculate_folder_sizes(folder):
    folder_sizes = []
    total = 0
    for entry in folder["entries"]:
        if "size" in entry:
            total += entry["size"]
        else:
            size, fsizes = calculate_folder_sizes(entry)
            total += size
            folder_sizes += fsizes
    folder_sizes.append(total)
    return total, folder_sizes


with open(f"{os.getcwd()}/2022/input/day07.txt") as f:
    fs = {"name": "/", "entries": [], "parent": {}}
    FS = fs
    cmds = f.read().split("$")[2:]
    for cmd in cmds:
        cmd = cmd.strip()
        data = [x.strip() for x in cmd.split("\n")]
        cmd = data[0]

        if cmd.startswith("cd"):
            dir = cmd.split(" ")[1]
            if dir == "..":
                fs = fs["parent"]
            else:
                fs = find(fs, dir)
        else:
            for entry in data[1:]:
                first, second = entry.split(" ")
                if first == "dir":
                    fs["entries"].append({"name": second, "entries": [], "parent": fs})
                else:
                    fs["entries"].append({"name": second, "size": int(first)})
    used, sizes = calculate_folder_sizes(FS)
    total = 0
    unused = TOTAL_FS_SIZE - used
    removable = NECESSARY_UNUSED - unused
    for size in sizes:
        if size <= 100000:
            total += size
    print(f"Part One: {total}")
    for size in sorted(sizes):
        if size >= removable:
            print(f"Part Two: {size}")
            break
