w = int(input())
x = int(input())
y = int(input())
z = int(input())
sumNum = 10
optionNum = 4


def oneTruth(truths, index):
    return all(map(lambda p: p[1] if p[0] == index else not p[1], enumerate(truths)))


def countAns(ans):
    return map(lambda n: sum(map(lambda i: i == n, ans)), range(optionNum))


asserts = [
    # 4
    lambda ans: oneTruth(
        [
            ans[1] == ans[5],
            ans[2] == ans[7],
            ans[1] == ans[9],
            ans[6] == ans[10],
        ],
        ans[4],
    ),
    # 6
    lambda ans: oneTruth(
        map(
            lambda l: len(set(map(lambda i: ans[i], l))) == 1,
            map(lambda p: [p[0], p[1], 8], [(2, 4), (1, 6), (3, 10), (5, 9)]),
        ),
        ans[6],
    ),
    # 7
    lambda ans: oneTruth(
        map(lambda m: min(countAns(ans)) == list(countAns(ans))[m], [2, 1, 0, 3]),
        ans[7],
    ),
    # 8
    lambda ans: oneTruth(
        map(lambda m: abs(ans[m] - ans[1]) != 1, [w, x, y, z]), ans[8]
    ),
    # 9
    lambda ans: oneTruth(
        map(lambda m: (ans[1] == ans[6]) ^ (ans[m] == ans[5]), [6, 10, 2, 9]), ans[9]
    ),
    # 10
    lambda ans: max(countAns(ans)) - min(countAns(ans)) == [3, 2, 4, 1][ans[10]],
]

for possibilityNum in range(optionNum**sumNum):
    possibilityList = list(
        map(lambda i: (possibilityNum // optionNum**i) % optionNum, range(sumNum))
    )
    possibilityList.insert(0, -1)
    if all(map(lambda f: f(possibilityList), asserts)):
        del possibilityList[0]
        print("".join(map(lambda n: chr(ord("A") + n), possibilityList)))
