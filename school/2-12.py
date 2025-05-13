import math

a = float(input())
b = float(input())

if a != 0:
    a2 = a * 2
    d = b ** 2 - 4 * a * float(input())
    n = -b / a2
    m = math.sqrt(abs(d)) / abs(a2)
    if d >= 0:
        print("%.2f"%(n + m))
        print("%.2f"%(n - m))
    else:
        print("%.2f+%.2fi"%(n, m))
        print("%.2f-%.2fi"%(n, m))
else:
    if b == 0:
        print("Not an equation")
    else:
        print("%.2f"%(-float(input()) / b))


