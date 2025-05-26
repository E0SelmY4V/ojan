"""
题目（Description）：

输入一个三位数，求各位数的立方之和并输出。

输入（Input）：

任意一个三位数

输出（Output）：

各位数的立方之和
"""

n = int(input())
a = n % 10
b = (n // 10) % 10
c = n // 100


def f(n):
    return n * n * n


print(f(a) + f(b) + f(c))
