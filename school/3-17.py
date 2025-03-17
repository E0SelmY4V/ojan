x = int(input())
w = int(input())
n = int(input())
print(x * sum(map(lambda i: i % 7 < 5, range(w - 1, w + n - 1))))
