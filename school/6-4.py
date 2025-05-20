print(
    "奇数和:%d\n偶数和:%d"
    % (
        (lambda f: lambda *a: f(f, *a))(
            lambda s, n: (
                lambda a: (
                    n
                    if a == -1
                    else s(s, (n[0] + a, n[1]) if a % 2 else (n[0], n[1] + a))
                )
            )(int(input()))
        )
    )((0, 0))
)
