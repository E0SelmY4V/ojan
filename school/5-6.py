from itertools import count, takewhile, chain

feb = lambda n: n if n < 2 else feb(n - 1) + feb(n - 2)
n = int(input())
l = list(takewhile(lambda i: i <= n, map(feb, count())))
print(",".join(map(str, chain(l, [sum(l), sum(l) // len(l)]))))
