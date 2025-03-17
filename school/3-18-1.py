for i in range(4):
    for j in range(4):
        if all(
            map(
                lambda q: q[1] if q[0] != j else not q[1],
                enumerate([i != 0, i == 2, i == 3, j == 2]),
            )
        ):
            print(chr(i + ord("A")))
