
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
    pass

def part1(data):
    
    ans = 0
    for line in data:
        a, b = line[:len(line)//2], line[len(line)//2:]
        a = set(a)
        b = set(b)
        for c in a:
            if c in b:
                ans += ((ord(c) - ord('a') + 1) if str(c).islower() else (ord(c) - ord('A') + 1 + 26))
                break
    return ans
    

def part2(data):
    ans = 0
    
    for i in range(len(data)//3):
        
        a = set(data[i*3])
        b = set(data[i*3+1])
        c = set(data[i*3+2])

        commons = a.intersection(b).intersection(c)
        # only has 1 common character
        
        for char in commons:
            ans += ((ord(char) - ord('a') + 1) if str(char).islower() else (ord(char) - ord('A') + 1 + 26))        
    return ans

test_file = 'test.txt'

input_file = 'input.txt'

print("---- Part 1 ----")
test_input = read_input(test_file)
test_output = part1(test_input)
part1_expected_test_output = 157
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
part2_expected_test_output = 70
test_output = part2(test_input)
if test_output == part2_expected_test_output:
    print('Test passed')
    output = part2(input)
    print('Part 2 Output:', output)
else:
    print('Test failed')
    print('Expected:', part2_expected_test_output)
    print('Got:', test_output)
