from itertools import chain

s = input()
print(max(filter(lambda n: n.isnumeric(), chain(*map(lambda i: map(
    lambda j: s[j: i + 1], range(i + 1)), range(len(s))))), key=len))
