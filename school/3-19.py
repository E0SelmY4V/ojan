from random import randint

l = [1, 2, 3, 4, 5]
rand = lambda: l[randint(0, len(l) - 1)]
n = int(input())
t = lambda _: rand() + rand() + rand() == n
c = 99999
print("%.2lf" % (sum(map(t, range(c))) / c))
