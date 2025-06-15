a = input()
b = input()
try:
    print(a + b[list(map(lambda i: a[-(i + 1):] == b[:i + 1],
          range(min(len(a), len(b))))).index(True) + 1:])
except:
    pass
