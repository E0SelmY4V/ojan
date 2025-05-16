def a(s):
    if s == "":
        return ("", "")
    l = a(s[1:])
    return (l[0], l[1] + s[0]) if s[0] in l[0] else (l[0] + s[0], l[1])


s = eval(input())
k = a(s)[1]
print(sorted(filter(lambda n: n not in k, s)))
