lower_ord = ord("a")
upper_ord = ord("A")
print(
    "".join(
        map(
            chr,
            map(
                lambda s: (
                    ((s[0] - s[1] * lower_ord - s[2] * upper_ord + 3) % 26)
                    + s[1] * lower_ord
                    + s[2] * upper_ord
                    if s[1] | s[2]
                    else s[0]
                ),
                map(lambda n: (ord(n), n.islower(), n.isupper()), input()),
            ),
        )
    )
)
