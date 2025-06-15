from itertools import combinations
n = eval(input())
m = int(input())
print((["[%d,%d]" % tuple(map(n.index, [a, b]))
      for a, b in combinations(n, 2) if a + b == m] + [False])[0])
