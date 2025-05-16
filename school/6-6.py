from itertools import groupby

print(max(map(lambda n: (n[0], sum(map(lambda _: 1, n[1]))), groupby(sorted(eval(input())))), key=lambda n: n[1])[0])
