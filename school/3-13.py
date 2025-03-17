def p(n):
    return n * p(n - 1) if n != 0 else 1

a = input()
r = sum(map(p, map(int, a)))
print(r)
print("YES" if r == int(a) else "NO")
