print(
    (lambda f: f(f, list(range(1, int(input()) + 1)), int(input()) - 1))(
        lambda s, z, m, i=0, c=[]: (
            (lambda i: s(s, z[:i] + z[i + 1 :], m, i, c + [z[i]]))((m + i) % len(z))
            if len(z)
            else " ".join(map(str, c))
        )
    )
)
