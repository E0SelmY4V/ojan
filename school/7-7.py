from math import gcd


class d:
    def __init__(self, n: str):
        a, b = map(int, n.split("/"))
        c = gcd(a, b)
        self.n = (a // c, b // c)

    __str__ = lambda self: str(self.n[0]) if self.n[1] == 1 else "%d/%d" % self.n
    __hash__ = lambda self: self.n.__hash__()
    __lt__ = lambda self, other: self.n[0] / self.n[1] < other.n[0] / other.n[1]
    __eq__ = lambda self, other: self.n == other.n


a = set(map(d, input().split(" ")))
b = set(map(d, input().split(" ")))
c = lambda n: print(" ".join(map(str, sorted(n))))
c(a & b)
c(a | b)
c(a - b)
