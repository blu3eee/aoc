
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
    

def convert_data(data):
    data = [line.split(",") for line in data]
    res = []
    for line in data:
        e1 = line[0].split("-")
        e2 = line[1].split("-")
        res.append(((int(e1[0]), int(e1[1])), (int(e2[0]), int(e2[1]))))
    return res

def part1(data):
    data = convert_data(data)
    ans = 0
    for line in data:
        ((s1, e1), (s2, e2)) = line

        ans+=  1 if (s1 <= s2 and e1 >= e2) or (s1 >= s2 and e1 <= e2) else 0
    return ans

def part2(data):
    data = convert_data(data)
    ans = 0
    for line in data:
        ((s1, e1), (s2, e2)) = line

        ans+=  1 if (s1 <= s2 and e1 >= e2) or (s1 >= s2 and e1 <= e2) or (s1 <= s2 and e1 >= s2) or (s1 <= e2 and e1 >= e2) else 0
    return ans
    pass

test_file = 'test.txt'

input_file = 'input.txt'

print("---- Part 1 ----")
test_input = read_input(test_file)
test_output = part1(test_input)
part1_expected_test_output = 2
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
part2_expected_test_output = 4
test_output = part2(test_input)
if test_output == part2_expected_test_output:
    print('Test passed')
    output = part2(input)
    print('Part 2 Output:', output)
else:
    print('Test failed')
    print('Expected:', part2_expected_test_output)
    print('Got:', test_output)
