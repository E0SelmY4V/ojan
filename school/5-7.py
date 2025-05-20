print(
    (
        (lambda f: lambda *a: f(f, *a))(
            lambda s, n: n if n <= 2 else s(s, n - 1) + s(s, n - 2)
        )
    )(int(input()))
)
