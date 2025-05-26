from datetime import datetime, timedelta
from itertools import chain, dropwhile, pairwise, accumulate, starmap
from functools import reduce

begin = 3
begin_price = 13
price = 2.3
multiple = 1.2
tax = 1

(
    lambda deal, calc: print(
        "%.2lf"
        % (
            lambda money: (
                sum(starmap(lambda a, b: a * b, zip([price, price * multiple], money)))
                + begin_price
                + tax
                if sum(money) > begin
                else begin_price
            )
        )(
            (lambda f: f(f))(
                lambda s, *r: (
                    s(s, *r, deal(*filter(bool, input().split(" "))))
                    if not len(r) or r[-1][1]
                    else calc(r)
                )
            )
        )
    )
)(
    lambda t, v: (datetime(*[1] * 3, *map(int, t.split(":"))), int(v)),
    (
        lambda split_price, splited_day, roading, coll: (
            lambda droping: lambda r: tuple(
                reduce(
                    coll,
                    starmap(lambda n, s, q: (n, min(s, q)), droping(r)),
                    (0, 0),
                )
            )
        )(
            lambda r: dropwhile(
                lambda n: n[2] <= 0,
                accumulate(
                    starmap(roading, chain(*starmap(split_price, splited_day(r)))),
                    lambda a, b: (b[0], b[1], a[2] + b[1]),
                    initial=(True, 0, -begin),
                ),
            )
        )
    )(
        (
            lambda dawn, late: lambda tb, tn, v: (
                [(dawn - tb, v, False), (late - dawn, v, True), (tn - late, v, False)]
                if tb < dawn and late <= tn
                else ([(tn - tb, v, False)] if tb < dawn and tn < dawn else [])
                + (
                    [(dawn - tb, v, False), (tn - dawn, v, True)]
                    if tb < dawn and dawn <= tn
                    else []
                )
                + ([(tn - tb, v, True)] if dawn <= tb and tn < late else [])
                + (
                    [(late - tb, v, True), (tn - late, v, False)]
                    if tb < late and late <= tn
                    else []
                )
                + ([(tn - tb, v, False)] if late <= tb and late <= tn else [])
            )
        )(*(datetime(*[1] * 3, i) for i in [5, 23])),
        lambda r: chain(
            *(
                (
                    [(tb, tn, v)]
                    if tn > tb
                    else [
                        (tb, datetime(1, 1, 2), v),
                        (datetime(1, 1, 2) - timedelta(1), tn, v),
                    ]
                )
                for (tb, v), (tn, _) in pairwise(r)
            )
        ),
        lambda t, v, n: (n, t.total_seconds() * v / 3600),
        lambda r, n: ((r[0] + n[1], r[1]) if n[0] else (r[0], r[1] + n[1])),
    ),
)
