def isNum(s):
    s = s.split("+", maxsplit=1)
    if len(s) == 2:
        return s[1][-1].isalpha() & isNum(s[0]) & isNum(s[1][:-1])
    else:
        s = s[0].split(".", maxsplit=1)
        if len(s) == 2:
            return isNum(s[0]) & isNum(s[1])
        else:
            return s[0].isnumeric()
