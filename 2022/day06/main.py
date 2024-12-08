
def read_input(file):
    # read all the lines in the file and return them as a list
    with open(file, 'r') as f:
        data = f.readlines()
    # strip the newline character from the end of each line
    data = [line.strip() for line in data]
    return data

def write_output(file, data):
    with open(file, 'w') as f:
        f.write(data)

def part1(data):
    ans = []
    for line in data:
        p1, p2 = 0, 1
        while p2 < len(line):
            while line[p2] in line[p1:p2] and p1 <= p2:
                p1+=1
            p2+=1
            if p2-p1 == 4:
                ans.append(p2)
                break
            
        

    return ans

def part2(data):
    ans = []
    for line in data:
        p1, p2 = 0, 1
        while p2 < len(line):
            
            while line[p2] in line[p1:p2] and p1 <= p2:
                p1+=1
            p2+=1
            if p2-p1 == 14:
                ans.append(p2)
                break
            
        
    return ans

test_file = 'test.txt'

input_file = 'input.txt'

print("---- Part 1 ----")
test_input = read_input(test_file)
test_output = part1(test_input)
part1_expected_test_output = [7,5,6,10,11]
if test_output == part1_expected_test_output:
    print('Test passed')
    input = read_input(input_file)
    print(test_input)
    output = part1(input)
    print('Part 1 Output:', output)
else:
    print('Test failed')
    print('Expected:', part1_expected_test_output)
    print('Got:', test_output)
    
# part 2
print("---- Part 2 ----")
part2_expected_test_output = [19,23,23,29,26]
test_output = part2(test_input)
if test_output == part2_expected_test_output:
    print('Test passed')
    output = part2(input)
    print('Part 2 Output:', output)
else:
    print('Test failed')
    print('Expected:', part2_expected_test_output)
    print('Got:', test_output)
