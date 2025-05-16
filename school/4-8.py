def cate(n):
    if n.islower():
        return 0
    elif n.isupper():
        return 1
    elif n.isdigit():
        return 2
    elif n.isspace():
        return 3
    else:
        return 4


l = list(map(cate, input()))
print(" ".join(map(str, map(lambda i: sum(map(lambda j: j == i, l)), range(5)))))
