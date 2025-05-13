y = int(input())
a = y % 10
b = (y // 10) % 10
c = y // 100
if a**3 + b**3 + c**3 == y:
    print("YES")
else:
    print("NO")
