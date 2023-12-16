from itertools import product
from os import PathLike


def check_rejected_num_log(filename: PathLike):
    with open(filename, 'r') as log:
        text = log.readlines()

    height = len(text)
    width = len(text[0].strip())

    for ln, line in enumerate(text):
        for ch, char in enumerate(line.strip()):
            if not char.isdigit() and not char == ".":
                x_range = range(max(ch - 1, 0), min(ch + 1, width))
                y_range = range(max(ln - 1, 0), min(ln + 1, height))
                for y, x in product(y_range, x_range):
                    if text[y][x].isdigit():
                        print(f"{y}:{x}")

if __name__ == "__main__":
    check_rejected_num_log("./logs/d3p1.log")

