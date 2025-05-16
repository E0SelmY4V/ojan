import math

print(" ".join(map(str, filter(lambda n: all(map(lambda i: n % i, range(2, int(math.sqrt(n)) + 1))), range(100, 300),))) + " ")
