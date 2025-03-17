from itertools import takewhile, count


def f(n):
    return 1 if n == 1 or n == 2 else f(n - 1) + f(n - 2)


print(";".join(map(str, takewhile(lambda n: n < 100, map(f, count(1))))) + ";")
