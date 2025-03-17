n = int(input())
if n == 0:
	n = 1
c = n
for i in range(1, n):
	c *= i
print(c)
