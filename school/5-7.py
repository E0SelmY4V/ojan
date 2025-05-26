print(
    (lambda f: f(f, int(input())))(
        lambda s, n: n if n <= 2 else s(s, n - 1) + s(s, n - 2)
    )
)
