from itertools import chain

s = eval(input())
print(list(chain(*zip(reversed(sorted(s[::2])), s[1::2]))))
