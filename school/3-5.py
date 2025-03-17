n = int(input())

while n != 1:
	if n % 2:
		n = n * 3 + 1
	else:
		n //= 2
	print(n)
