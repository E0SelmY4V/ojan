"""
题目（Description）：

输入一个三位数，将它反向输出。

输入（Input）：

任意一个三位数

输出（Output）：

反向三位数
"""

n = int(input())
a = n % 10
b = (n // 10) % 10
c = n // 100
print(a * 100 + b * 10 + c)
