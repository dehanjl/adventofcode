import argparse
from itertools import combinations
from typing import List, Tuple

from shapely.geometry import Polygon, box


def parse_input(path: str) -> List[Tuple[int, int]]:
    """
    Parse the input file lines like "7,1" into a list of tuples (col, row).
    This matches Shapely's (x, y) convention directly.
    """
    tiles: List[Tuple[int, int]] = []
    with open(path, encoding="utf-8") as fh:
        for raw in fh:
            line = raw.strip()
            if not line or line.startswith("#"):
                continue
            parts = line.split(",", 1)
            if len(parts) != 2:
                raise ValueError(f"Invalid line in input: {line!r}")
            c_str, r_str = parts
            tiles.append((int(c_str), int(r_str)))
    return tiles


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("path", help="Path to input file to parse")
    args = parser.parse_args()

    tiles = parse_input(args.path)

    max_area = 0

    # tiles are (col, row) = (x, y), matching Shapely's convention
    poly = Polygon(tiles)

    def rect_area(p: Tuple[int, int], q: Tuple[int, int]) -> int:
        return (abs(p[0] - q[0]) + 1) * (abs(p[1] - q[1]) + 1)

    for loc1, loc2 in combinations(tiles, 2):
        rect = box(
            min(loc1[0], loc2[0]),
            min(loc1[1], loc2[1]),
            max(loc1[0], loc2[0]),
            max(loc1[1], loc2[1]),
        )
        if poly.covers(rect):
            max_area = max(max_area, rect_area(loc1, loc2))

    print(f"Day 9 Part 2: {max_area}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
