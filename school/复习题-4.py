y = int(input())
m = int(input())
d = int(input())

if (y % 4 == 0 and y % 100 != 0) or y % 400 == 0:
    w = 29
else:
    w = 28
l = [0, 31, w, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31]

for i in range(m):
    d += l[i]

print(d)
