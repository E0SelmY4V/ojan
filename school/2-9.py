import math

c = 10

l = math.ceil(float(input()))
l = max(0, l - 3)
c += 2 * min(10, l)
l = max(0, l - 10)
c += 3 * l

w = int(input())
if w < 5:
    w = 0
else:
    w = math.ceil(w / 5)
    
c += w * 2

print(c)
