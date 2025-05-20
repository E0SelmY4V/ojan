from itertools import count, takewhile, chain

n = int(input())
l = list(takewhile(lambda i: i <= n, map((lambda f: lambda *a: f(f, *a))(lambda s, n: n if n < 2 else s(s, n - 1) + s(s, n - 2)), count())))
print(",".join(map(str, chain(l, [sum(l), sum(l) // len(l)]))))
