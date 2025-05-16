# 这份代码必须用列表推导式

l = eval(input())
print("\n".join(map(lambda n: "下标： %d 值： %d" % n, zip(range(999), l))))
a = sum(l) / len(l)
print("平均值： %.1f" % a)
print([n for n in l if n > a])