from math import sin, cos, sqrt, pi

a = float(input())
b = float(input())
print((-b + sqrt(2 * a * sin(pi / 3) * cos(pi / 3))) / (2 * a))
