# 这份代码必须用列表推导式

l = [1, 2, 3, 4]
print(
    [a * 100 + b * 10 + c for a in l for b in l for c in l if len(set([a, b, c])) == 3]
)
