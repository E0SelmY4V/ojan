import random

xyList = list(map(lambda _: int(input()), range(4)))
n = int(input())
m = int(input())
count = 5000000


def exchangeOne(xyList):
    randed = random.randint(-1, 2)
    if randed % 2:
        xyList[0] -= randed
        xyList[1] += randed
        xyList[2] += randed
        xyList[3] -= randed


def test(xyList):
    for _ in range(n):
        exchangeOne(xyList)
    return xyList[0] == m


print("%.2f" % (sum(map(lambda _: test(xyList[:]), range(count))) / count))
