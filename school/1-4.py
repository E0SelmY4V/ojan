def m(r):
    return 3.14159 * r * r

a = float(input())
b = float(input())
s = m(a) - m(b)
print(round(s, 2))
