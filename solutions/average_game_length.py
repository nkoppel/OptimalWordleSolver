import sys

with open(sys.argv[1]) as file:
    stats = [0] * 6
    average = 0
    denom = 0

    for line in file:
        n = len(line.split()) // 2
        average += n
        stats[n - 1] += 1
        denom += 1

    print(stats)
    print(average / denom)
