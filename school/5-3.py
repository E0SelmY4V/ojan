f = lambda x: x * x * x - 20 * x - 1
a = float(input())
b = float(input())
if a > b:
    [a, b] = [b, a]
for _ in range(5000):
	c = (a + b) / 2
	if (f(a) < 0) == (f(c) > 0):
		b = c
	else:
		a = c
print("%.2f"%a)
