s = input()
n = (float if "." in s else int)(s)
print(*map(lambda i: n**i, range(6)))
