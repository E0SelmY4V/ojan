jump = lambda n: n if n <= 2 else jump(n - 1) + jump(n - 2)
print(jump(int(input())))