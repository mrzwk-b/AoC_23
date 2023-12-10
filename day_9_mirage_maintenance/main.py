def parseLine(line):
    sequence = []
    currentNumber = 0
    negative = False
    for char in line:
        if char == '\n':
            sequence.append(currentNumber * (-1 if negative else 1))
            return sequence
        elif char == ' ':
            sequence.append(currentNumber * (-1 if negative else 1))
            currentNumber = 0
            negative = False
        elif char == '-':
            negative = True
        else:
            currentNumber = currentNumber * 10 + int(char)

def whatComesNext(sequence):
    allZeros = False
    allSequences = [sequence]
    row = 0
    while not allZeros:
        allZeros = True
        newSequence = []
        for i in range(len(allSequences[row]) - 1):
            newValue = allSequences[row][i+1] - allSequences[row][i]
            allZeros &= newValue == 0
            newSequence.append(newValue)
        allSequences.append(newSequence)
        row += 1
    total = 0
    for s in allSequences:
        total += s[-1] if len(s) > 0 else 0
    return total

def whatComesFirst(sequence):
    allZeros = False
    allSequences = [sequence]
    row = 0
    while not allZeros:
        allZeros = True
        newSequence = []
        for i in range(len(allSequences[row]) - 1):
            newValue = allSequences[row][i+1] - allSequences[row][i]
            allZeros &= newValue == 0
            newSequence.append(newValue)
        allSequences.append(newSequence)
        row += 1
    total = 0
    for i in range(len(allSequences)):
        total += (allSequences[i][0] if len(allSequences[i]) > 0 else 0) * (1 if i % 2 == 0 else -1)
    return total

file = open('..\\..\\AoC_23_aux\\inputs\\day_9_input.txt')
histories = file.readlines()
total = 0
for h in histories:
    #total += whatComesNext(parseLine(h))
    total += whatComesFirst(parseLine(h))
print(total)