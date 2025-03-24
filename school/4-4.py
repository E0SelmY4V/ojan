try:
    s = input()
    if s[0] == "Y":
        h = 1 / 6
        n = "S"
    elif s[0] == "S":
        h = 6
        n = "Y"
    else:
        raise 0
    print("%s%.2f" % (n, int(s[1:]) * h))
except:
    print("error!")
