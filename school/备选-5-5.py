def h(n, f, t, m):
    if n == 1:
        yield (f, t)
    else:
        yield from h(n - 1, f, m, t)
        yield (f, t)
        yield from h(n - 1, m, t, f)


print(*("%s-->%s " % n for n in h(int(input()), "A", "C", "B")), sep="")
