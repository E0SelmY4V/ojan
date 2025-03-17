n = int(input())
c = 2
for i in range(1, n + 1):
	c -= ((i % 2) * 2 - 1) * (1 / i)
print("%.3f"%c)