from functools import reduce
from itertools import groupby

c: dict[tuple[int, int], int] = {}
d: dict[int, set[tuple[int, int]]] = {1: set()}
m = 1
while True:
    n = input()
    if n == "00000":
        break
    k = set(map(int, n[:-1].split(" ")))
    for i in k:
        for j in k:
            if i < j:
                if (i, j) in c:
                    c[(i, j)] += 1
                else:
                    c[(i, j)] = 1
                if c[(i, j)] == m:
                    d[m].add((i, j))
                elif c[(i, j)] > m:
                    m = c[(i, j)]
                    d[m] = set([(i, j)])
                    del d[m - 1]
print("\n".join("%d %d" % n for n in sorted(d[m])))
