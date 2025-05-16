def r(n=(0, 0)):
    a = int(input())
    return n if a == -1 else r((n[0] + a, n[1]) if a % 2 else (n[0], n[1] + a))


print("奇数和:%d\n偶数和:%d" % r())
