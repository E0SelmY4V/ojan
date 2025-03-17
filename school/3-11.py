n = int(input())

r = "".join(
    map(
        lambda i: "".join(map(lambda k: str(k + (n - sum(range(i))) // i) + " ", range(i))) + "\n\n",
        filter(
            lambda i: (
                n % i == 0 and n // i > i // 2
                if i % 2
                else (n - i // 2) % i == 0 and (n + i // 2) // i > i // 2
            ),
            reversed(range(2, n)),
        ),
    )
)
print(r if r else "no")

# 2n + 1
# 3n
# 4n + 2
# 5n
# 6n + 3
# (s+m)*(m-s+1)/2
