def is_phone(i):
    j = i[2]
    if i.startswith("13"):
        if j == "1" or j == "2" or j == "3" or j == "0":
            return False
        return len(i) == 11
    elif i.startswith("15"):
        if j == "3" or j == "4" or j == "5" or j == "6" or j == "7":
            return False
        return len(i) == 11
    else:
        return False


for i in eval(input()):
    if is_phone(i):
        print(i)
