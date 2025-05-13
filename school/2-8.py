import math

a = int(input())
if a == 0:
    print("Data error!")
else:
    b = int(input())
    c = int(input())

    d = b ** 2 - 4 * a * c
    if d < 0:
        print("No solution!")
    else:
        if d == 0:
            print(round(-float(b) / (2 * float(a)), 1))
        else:
            print(
                round((-float(b) + math.sqrt(d)) / (2 * float(a)), 1),
                round((-float(b) - math.sqrt(d)) / (2 * float(a)), 1),
                sep=" "
            )
