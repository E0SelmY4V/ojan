sumNum = 10
optionNum = 4


def diffFrom(ans, options, index):
    uni = options[index]
    del options[index]
    return len(set(map(lambda i: ans[i], options))) == 1 and ans[uni] != ans[options[0]]


def oneTruth(truths, index):
    return all(map(lambda p: p[1] if p[0] == index else not p[1], enumerate(truths)))


def countAns(ans):
    return map(lambda n: sum(map(lambda i: i == n, ans)), range(optionNum))


asserts = [
    # 2
    lambda ans: [2, 3, 0, 1][ans[2]] == ans[5],
    # 3
    lambda ans: diffFrom(ans, [3, 6, 2, 4], ans[3]),
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
    # 5
    lambda ans: oneTruth(map(lambda i: ans[i] == ans[5], [8, 4, 9, 7]), ans[5]),
    # 6
    lambda ans: oneTruth(
        map(
            lambda n: ans[n[0]] == ans[n[1]] == ans[8],
            [(2, 4), (1, 6), (3, 10), (5, 9)],
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
        map(lambda m: abs(ans[m] - ans[1]) != 1, [7, 5, 2, 10]), ans[8]
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
