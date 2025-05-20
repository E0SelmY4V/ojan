from random import randint
from itertools import groupby

t = {
    (8, 4): 100,
    (8, 3, 1): 10,
    (8, 2, 2): 10,
    (7, 5): 20,
    (7, 4, 1): 2,
    (7, 3, 2): 2,
    (6, 6): 20,
    (6, 5, 1): 1,
    (6, 4, 2): 1,
    (6, 3, 3): 1,
    (5, 5, 2): 1,
    (5, 4, 3): -10,
    (4, 4, 4): 1,
}


def x(a, k):
    for i in (len(a) - i - 1 for i in range(k)):
        r = randint(0, i)
        a[r], a[i] = a[i], a[r]
    return a[-k:]


c = lambda a: tuple(
    sorted(map(lambda n: len(list(n[1])), groupby(sorted(a))), reverse=True)
)
z = list(map(lambda i: i // 8, range(24)))
j = 99999
print("%.2lf" % (sum(map(lambda _: t[c(x(z[:], 12))], range(j))) / j))
