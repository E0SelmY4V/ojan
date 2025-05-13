f = lambda x: x**4 - 33 * x**3 + 217 * x**2 + 825 * x - 6050
a = float(input())
b = float(input())
if a > b:
    [a, b] = [b, a]
if f((a + b) / 2) == 0:
    a = (a + b) / 2
else:
    for _ in range(5000):
        c = (a + b) / 2
        if (f(a) < 0) == (f(c) > 0):
            b = c
        else:
            a = c
print("%.1f" % a)
