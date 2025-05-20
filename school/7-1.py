from random import randint
from itertools import groupby

n = int(input())
m = int(input())


j = lambda _: len(list(filter(lambda i: i > 1, map(lambda i: len(list(i[1])), groupby(sorted(randint(1, 365) for _ in range(n))))))) >= m


c = 49999
print("%.2lf" % (sum(map(j, range(c))) / c))
