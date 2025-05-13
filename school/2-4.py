y = int(input())
n = 365
if y % 400 == 0 or (y % 4 == 0 and y % 100 != 0):
    n += 1
print(n)
