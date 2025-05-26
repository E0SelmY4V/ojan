p = eval(input())
s = (lambda f: f(f, p))(
    lambda s, k: (
        ("", "")
        if k == ""
        else (lambda l: ((l[0], l[1] + k[0]) if k[0] in l[0] else (l[0] + k[0], l[1])))(
            s(s, k[1:])
        )
    )
)[1]
print(sorted(filter(lambda n: n not in s, p)))
