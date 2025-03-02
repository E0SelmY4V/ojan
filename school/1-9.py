import math

a = float(input())
b = float(input())
c = float(input())

d = b * b - 4 * a * c;
x1 = (-b - math.sqrt(d)) / (2 * a)
x2 = (-b + math.sqrt(d)) / (2 * a)
print("{:.2f},{:.2f}".format(x1, x2))
