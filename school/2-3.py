a = int(input())
b = max(0, a - 3500)

if b < 1500:
    c = 0.03
elif b < 4500:
    c = 0.1
elif b < 9000:
    c = 0.2
elif b < 35000:
    c = 0.25
elif b < 55000:
    c = 0.3
else:
    c = 0.35

print(round(b*c))
