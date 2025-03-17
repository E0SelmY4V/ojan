x = int(input())
y = int(input())
z = int(input())
sayings = [
    (("B", x), ("A", 3)),
    (("B", x), ("E", z)),
    (("C", y), ("D", x)),
    (("C", 5), ("D", 3)),
    (("E", z), ("A", y)),
]
persons = list(map(chr, range(ord("A"), ord("F"))))

for possibilityStr in range(2**5):
    possibilityList = list(enumerate(map(int, bin(possibilityStr)[2:].rjust(5, "0"))))
    unableSets = list(map(lambda _: set(), range(5)))
    for personIndex, which in possibilityList:
        for unableIndex in filter(
            lambda l: l != sayings[personIndex][which][1] - 1, range(0, 5)
        ):
            unableSets[unableIndex].add(sayings[personIndex][which][0])
        unableSets[sayings[personIndex][1 - which][1] - 1].add(
            sayings[personIndex][1 - which][0]
        )
    order = ["Nobody"] * 5
    ableSets = list(map(lambda h: set(persons).difference(h), unableSets))
    for index, ableSet in [
        item
        for sublist in [
            filter(lambda ableSet: len(ableSet) == 1, enumerate(ableSets)),
            enumerate(ableSets),
        ]
        for item in sublist
    ]:
        for person in ableSet:
            if person not in order:
                order[index] = person
    if "Nobody" not in order:
        orderMap = dict(map(reversed, enumerate(order)))
        print(" ".join(map(lambda person: str(orderMap[person] + 1), persons)) + " ")
