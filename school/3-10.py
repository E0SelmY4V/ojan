from random import seed, randint

seed(125)
n = randint(0, 100)
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
	print("%d times, you got it!" % c - 1)
	break

