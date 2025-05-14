def h(n, f, t, m):
    if n == 1:
        yield (f, t)
    else:
        yield from h(n - 1, f, m, t)
        yield (f, t)
        yield from h(n - 1, m, t, f)


try:
    print("%s->%s" % list(h(int(input()), "a", "c", "b"))[int(input()) - 1])
except IndexError:
    pass
