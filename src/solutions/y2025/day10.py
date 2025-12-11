import argparse
import re

import numpy as np
from scipy.optimize import Bounds, LinearConstraint, milp


def parse_line(line: str):
    # Extract all parentheses groups (for A rows)
    paren_groups = re.findall(r"\(([^)]*)\)", line)

    # Extract the curly braces group (for t)
    t_match = re.search(r"\{([^}]*)\}", line)
    t = np.array([int(x) for x in t_match.group(1).split(",")])

    num_cols = len(t)
    num_rows = len(paren_groups)

    A = np.zeros((num_rows, num_cols), dtype=int)

    for i, group in enumerate(paren_groups):
        if group:  # handle non-empty groups
            indices = [int(x) for x in group.split(",")]
            for idx in indices:
                A[i, idx] = 1

    return A, t


def solve_machine(A: np.ndarray, t: np.ndarray) -> int:
    n = A.shape[0]  # number of variables in v

    # Minimize sum(v) → coefficients are all 1
    c = np.ones(n)

    # Constraint: v @ A = t  →  A.T @ v = t
    constraints = LinearConstraint(A.T, t, t)

    # v >= 0, integers
    bounds = Bounds(lb=0, ub=np.inf)
    integrality = np.ones(n)  # 1 = integer constraint

    result = milp(c, constraints=constraints, bounds=bounds, integrality=integrality)
    return int(result.fun)


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("path", help="Path to the input file to parse")
    args = parser.parse_args()

    machines = []

    with open(args.path, encoding="utf-8") as fh:
        for i, raw in enumerate(fh, start=1):
            line = raw.strip()
            if not line or line.startswith("#"):
                continue
            try:
                A, t = parse_line(line)
                machines.append((A, t))
            except Exception as exc:
                print(f"Error parsing line {i}: {exc}")
                continue

    total = 0
    for i, (A, t) in enumerate(machines, start=1):
        cost = solve_machine(A, t)
        total += cost

    print(f"Day 10 Part 2: {total}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
