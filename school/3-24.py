from random import randint

n = int(input())


def d(_):
    d = [[4, 2], [3, 4]]
    g = [[0, 0], [0, 0]]
    for i in range(2):
        for _ in range(n):
            r = randint(1, sum(d[i])) > d[i][0]
            g[1 - i][r] += 1
            d[i][r] -= 1
    return d[0][0] + g[0][0] == 4


c = 99999
print("%.2lf" % (sum(map(d, range(c))) / c))
