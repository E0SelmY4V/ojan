"""
描述

用户输入两个整数M和N，计算M和N的4种数学运算结果，并依次输出，结果间用空格分隔。

4种数学运算分别是：   M与N的和、M与N的乘积、M的N次幂、M除N的余数
"""

m = int(input())
n = int(input())
print(m + n, m * n, pow(m, n), m % n)
