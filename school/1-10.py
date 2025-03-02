s = float(input())
c = float(input())
t = float(input())

def out(n):
    print("%.1f"%n)

out(s * c)
out(s * c * t)
out((s * c) * (1 + t))
