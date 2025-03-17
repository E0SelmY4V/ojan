for i in range(100, 1000):
	n = str(i)
	if sum(map(lambda s: int(s) ** len(n), n)) == i:
		print(n)
