
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
    arr1 = []
    arr2 = []
    for line in data:
        [a, b] = line.split('   ')
        arr1.append(a)
        arr2.append(b)
    # sort arr1 and arr2
    arr1.sort()
    arr2.sort()
    return arr1, arr2

def part1(data):
    arr1, arr2 = convert_data(data)
    res = 0
    for i in range(len(arr1)):
        res += abs(int(arr1[i]) - int(arr2[i]))
    return res    

def part2_convert_data(data):
    dict1 = {}
    dict2 = {}
    for line in data:
        [a, b] = line.split('   ')
        dict1[a] = 1 if a not in dict1 else dict1[a] + 1
        dict2[b] = 1 if b not in dict2 else dict2[b] + 1
    
    return dict1, dict2

def part2(data):
    dict1, dict2 = part2_convert_data(data)
    
    res = 0
    for key in dict1:
        res += (dict1[key] * (dict2[key] if key in dict2 else 0))*int(key)
    return res

def main():
    test_file = 'test.txt'

    input_file = 'input.txt'

    print("---- Part 2 ----")
    test_input = read_input(test_file)
    test_output = part1(test_input)
    part1_expected_test_output = 11
    if test_output == part1_expected_test_output:
        print('Test passed')
    else:
        print('Test failed')
        print('Expected:', part1_expected_test_output)
        print('Got:', test_output)
        return

    input = read_input(input_file)
    print(test_input)
    output = part1(input)
    print('Part 1 Output:', output)

    # part 2
    print("---- Part 2 ----")
    part2_expected_test_output = 31
    test_output = part2(test_input)
    if test_output == part2_expected_test_output:
        print('Test passed')
    else:
        print('Test failed')
        print('Expected:', part2_expected_test_output)
        print('Got:', test_output)
        return
    
    output = part2(input)
    print('Part 2 Output:', output)
    

if __name__ == '__main__':
    main()