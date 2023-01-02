SMALL_DIRECTORY_MAX = 100_000
TOTAL_FS_SIZE = 7_000_0000
NECESSARY_UNUSED = 3_000_0000


def find(fs, name):
    for el in fs["entries"]:
        if el["name"] == name:
            return el
    assert False


def calculate_folder_sizes(folder):
    sizes = []
    total = 0
    for entry in folder["entries"]:
        if "size" in entry:
            total += entry["size"]
        else:
            size, nested_sizes = calculate_folder_sizes(entry)
            total += size
            sizes += nested_sizes
    sizes.append(total)
    return total, sizes


def build_filesystem(input: str):
    cur = {"name": "/", "entries": [], "parent": {}}
    root = cur
    elems = input.split("$")[2:]
    for el in elems:
        data = [s.strip() for s in el.strip().split("\n")]
        if data[0].startswith("cd"):
            dir = data[0].split(" ")[1]
            cur = cur["parent"] if dir == ".." else find(cur, dir)
        else:
            for entry in data[1:]:
                first, name = entry.split(" ")
                cur["entries"].append(
                    {"name": name, "entries": [], "parent": cur}
                    if first == "dir"
                    else {"name": name, "size": int(first)}
                )
    return root


def solve(input: str):
    root = build_filesystem(input)
    used, sizes = calculate_folder_sizes(root)
    total, removing = 0, 0
    unused = TOTAL_FS_SIZE - used
    removable = NECESSARY_UNUSED - unused
    for size in sizes:
        if size <= SMALL_DIRECTORY_MAX:
            total += size
    for size in sorted(sizes):
        if size >= removable:
            removing = size
            break
    return total, removing
