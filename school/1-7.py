n = int(input())
a = n % 10
b = (n // 10) % 10
c = n // 100
print(a * 100 + b * 10 + c)
