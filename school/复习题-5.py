from operator import eq
from itertools import starmap

y = "Beautiful is better than ugly."
print(y)
print(sum(starmap(eq, zip(y, input()))) / len(y))

