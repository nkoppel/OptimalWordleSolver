with open("2309_solution.txt") as file:
    average = 0
    denom = 0

    for line in file:
        average += len(line.split()) // 2
        denom += 1

    print(average / denom)
