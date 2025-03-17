import random

random.seed(125)
n = random.randint(0, 100)
c = 0

while True:
	c += 1
	try:
		i = int(input("Please input:"))
	except:
		print("Please input integer!")
		continue
	if i > n:
		print("Too big!")
		continue
	if i < n:
		print("Too small!")
		continue
	print(f"{c - 1} times, you got it!")
	break

