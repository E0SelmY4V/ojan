h = input() == 'H'
e = float(input())
s = 100
d = 1.0
if h:
    d -= 0.2
    s *= 10
if e >= s:
    d -= 0.1
print("%.2f"%(e * d))
