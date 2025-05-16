# 这份代码必须用列表推导式

for i in [
    "公鸡: %d 母鸡: %d 小鸡: %d" % (a, b, c)
    for a in range(101)
    for b in range(101)
    for c in range(101)
    if 15 * a + 9 * b + c == 300 and a + b + c == 100
]:
    print(i)
