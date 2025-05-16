lower_ord = ord("a")
upper_ord = ord("A")
print(
    "".join(
        [
            chr(
                (ord(i) - i.islower() * lower_ord - i.isupper() * upper_ord + 3) % 26
                + i.islower() * lower_ord
                + i.isupper() * upper_ord
                if i.islower() | i.isupper()
                else ord(i)
            )
            for i in input()
        ]
    )
)
