"""
题目（Description）：

输入一个华氏温度，要求输出摄氏温度。公式为 c=5(F-32)/9 。

输入（Input）：

一个华氏温度（浮点数）

输出（Output）：

摄氏温度（保留两位小数）
"""

F = float(input())
c = 5 * (F - 32) / 9
print("%.2f" % c)
