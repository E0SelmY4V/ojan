from random import randint

n = int(input())
l = range(1, n + 1)
rand = lambda: l[randint(0, len(l) - 1)]


def m(_):
    s = 0
    while s <= 50:
        s += rand()
    return s


c = 99999
print("%.1lf" % (sum(map(m, range(c))) / c))
