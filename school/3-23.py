from random import randint

n = int(input())
l = list(range(1, 6))


def rand(_):
    k = l
    s = 0
    for _ in range(3):
        r = randint(0, len(k) - 1)
        s += k[r]
        k = k[:r] + k[r + 1 :]
    return s == n


c = 999999
print("%.2lf" % (sum(map(rand, range(c))) / c))
