from sys import stdin

a = tuple(map(float, stdin))
print("%.2f" % (sum(a) / len(a)))
