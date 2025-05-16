s = input()
print("YES" if "10X98765432"[sum(map(lambda a: a[0] * a[1], zip([7, 9, 10, 5, 8, 4, 2, 1, 6, 3, 7, 9, 10, 5, 8, 4, 2], map(int, s[:17])))) % 11] == s[-1] else "NO")
