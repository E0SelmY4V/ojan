# 这份代码必须用列表推导式

l = [1, 9, 8, 7, 6, 5, 13, 3, 2, 1]
print(l)
print([i for i in l if not i % 2])
