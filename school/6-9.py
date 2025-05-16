l = eval(input())
print([i for i in l if i] + [0] * sum(map(lambda n: not n, l)))
