w = float(input())
h = float(input())
bmi = w / (h ** 2)
if bmi < 18:
    print("lower weight")
else:
    if bmi < 25:
        print("normal weight")
    else:
        if bmi < 27:
            print("higher weight")
        else:
            print("obesity")
