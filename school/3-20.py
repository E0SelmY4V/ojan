from random import randint

l = range(1, 21)
rand = lambda: l[randint(0, len(l) - 1)]
n = int(input())
t = lambda _: rand() + rand() + rand() + rand() + rand() == n
c = 99999
print("%.2lf" % (sum(map(t, range(c))) / c))
