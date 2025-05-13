from math import sqrt
from itertools import count, islice

is_prime = lambda n: all(map(lambda i: n % i, range(2, int(sqrt(n)) + 1)))
for i in islice(
    filter(lambda n: is_prime(n) & is_prime(int("".join(reversed(str(n))))), count(2)),
    int(input()),
):
    print(i, end=" ")
