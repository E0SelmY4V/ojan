from datetime import datetime, timedelta
from itertools import chain, dropwhile, pairwise, accumulate, starmap
from functools import reduce
from operator import mul

b, p, r, m, t = 3, 13, 2.3, 1.2, 1
(lambda *f: lambda r, u, *s: print("%.2lf" % u((lambda f: f(f))(r(f, *s)))))(
    lambda d, l: lambda b, n, v: (
        [(d - b, v, False), (l - d, v, True), (n - l, v, False)]
        if b < d and l <= n
        else ([(n - b, v, False)] if b < d and n < d else [])
        + ([(d - b, v, False), (n - d, v, True)] if b < d and d <= n else [])
        + ([(n - b, v, True)] if d <= b and n < l else [])
        + ([(l - b, v, True), (n - l, v, False)] if b < l and l <= n else [])
        + ([(n - b, v, False)] if l <= b and l <= n else [])
    ),
    (datetime(*[1] * 3, i) for i in (5, 23)),
    lambda o, r, d: chain(*(o(tb, tn, v, *d) for (tb, v), (tn, _) in pairwise(r))),
    lambda b, n, v, d, s: [(b, n, v)] if n > b else [(b, d, v), (d - s, n, v)],
    lambda t, v, n: (n, t.total_seconds() * v / 3600),
    (datetime(1, 1, 2), timedelta(1)),
)(
    lambda f, coll, droping, deal, calc: lambda s, *r: (
        s(s, *r, deal(*filter(bool, input().split(" "))))
        if not len(r) or r[-1][1]
        else calc(coll, dropwhile(lambda n: n[2] <= 0, droping(*f, r)))
    ),
    lambda o: sum(starmap(mul, zip([r, r * m], o))) + p + t if sum(o) > b else p,
    lambda r, n: ((r[0] + n[1], r[1]) if n[0] else (r[0], r[1] + n[1])),
    lambda split_price, sp, splited_day, o, roading, d, r: accumulate(
        starmap(roading, chain(*starmap(split_price(*sp), splited_day(o, r, d)))),
        lambda a, b: (b[0], b[1], a[2] + b[1]),
        initial=(True, 0, -b),
    ),
    lambda t, v: (datetime(*[1] * 3, *map(int, t.split(":"))), int(v)),
    lambda c, d: tuple(reduce(c, starmap(lambda n, s, q: (n, min(s, q)), d), (0, 0))),
)
