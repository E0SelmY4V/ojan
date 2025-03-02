n = int(input())
a = n % 10
b = (n // 10) % 10
c = n // 100
def f(n):
    return n * n * n
print(f(a) + f(b) + f(c))
