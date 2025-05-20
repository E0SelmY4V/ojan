from itertools import groupby

n = int(input())
c = lambda a, b: tuple((x + y) % n for (x, y) in zip(a, b))
print(
    (lambda f: f(f))(
        lambda s, p=[(0, n // 2)]: (
            "\n".join(
                " ".join(str(k[1] + 1) for k in a[1])
                for a in groupby(sorted(zip(p, range(n * n))), key=lambda n: n[0][0])
            )
            if len(p) == n**2
            else (lambda k: s(s, p + [c(k, (2, -1)) if k in p else k]))(
                c(p[-1], (-1, 1))
            )
        )
    )
)
