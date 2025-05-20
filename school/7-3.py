from random import randint
from itertools import groupby, chain


def x(a, k):
    for i in (len(a) - i - 1 for i in range(k)):
        r = randint(0, i)
        a[r], a[i] = a[i], a[r]
    return a[-k:]


c = lambda a: sorted(((len(list(b)), a) for (a, b) in groupby(sorted(a))), reverse=True)
z = list(chain(*([i] * int(input()) for i in range(3))))
j = 399999
g = int(input())
print(
    next(
        "_".join(map(str, m)) + " : %.2lf" % (u / j)
        for (u, m) in c(tuple(a for (a, _) in c(x(z[:], g))) for _ in range(j))
    )
)
