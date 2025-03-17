for i in range(0, 101):
    for j in range(0, 101):
        for k in range(0, 101):
            if i * 5 + j * 3 + k == 100 and all([i, j, k]) and i + j + k * 3 == 100:
                print("%d %d %d" % (i, j, k * 3))
