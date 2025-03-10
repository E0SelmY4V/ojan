import math

a = float(input())
b = float(input())
c = float(input())

def out(n):
    print("%.2f"%n)

def outi(a, b):
    print("%.2f+%.2fi"%(a, b))
    print("%.2f-%.2fi"%(a, b))

def sign(n):
    if n > 0:
        return 1
    elif n < 0:
        return -1
    else:
        return 0

if a == 0:
    if b == 0:
        print("Not an equation")
        exit()
    out(-c / b)
    exit()
d = b * b - 4 * a * c
if d >= 0:
    out((-b + sign(a) * math.sqrt(d)) / (2 * a))
    out((-b - sign(a) * math.sqrt(d)) / (2 * a))
else:
    outi(-b / (2 * a), abs(math.sqrt(abs(d)) / (2 * a)))

