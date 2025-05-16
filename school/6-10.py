def is_ip(n):
    s = list(n.split("."))
    if len(s) != 4:
        return False
    for i in s:
        if not i.isnumeric():
            return False
        if int(i) > 255:
            return False
    return True


print("Yes" if is_ip(input()) else "No")
