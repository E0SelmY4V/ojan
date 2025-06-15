print("".join(k[1] if " " in k else k[1].lower() for k in (lambda l, n: (
    tuple(l[i:i+n]) for i in range(len(l)-n+1)))(" %s " % input(), 3)))

