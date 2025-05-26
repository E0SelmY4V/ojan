print(
    "奇数和:%d\n偶数和:%d"
    % (lambda f: f(f, 0, 0))(
        lambda s, j, o: (
            lambda a: (j, o) if a < 0 else s(s, j + a * (a % 2), o + a * (1 - a % 2))
        )(int(input()))
    )
)
