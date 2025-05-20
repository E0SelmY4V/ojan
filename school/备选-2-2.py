print((lambda f: f(f, int(input())))(lambda s, a: a * s(s, a - 1) if a else 1))
