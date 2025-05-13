s = input()
powers = [7,9,10,5,8,4,2,1,6,3,7,9,10,5,8,4,2]
u = sum(map(lambda a: a[0] * a[1], zip(powers, map(int, s[:17]))))
o = "10X98765432"[u % 11]
print("YES" if o == s[-1] else "NO")