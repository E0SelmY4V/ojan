from random import randint

x1, y1, x2, y2, n, m = [int(input()) for _ in range(6)]


def d(_):
    d = [[x1, y1], [x2, y2]]
    g = [[0, 0], [0, 0]]
    for i in range(2):
        for _ in range(n):
            r = randint(1, sum(d[i])) > d[i][0]
            g[1 - i][r] += 1
            d[i][r] -= 1
    return d[0][0] + g[0][0] == m


c = 99999
print("%.2lf" % (sum(map(d, range(c))) / c))
