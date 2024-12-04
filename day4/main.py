
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
    
def part1(data: list[str]) -> int:
    # convert the data first
    res = 0
    for line in data:
        last_idx = 0
        
        while last_idx != -1 and last_idx < len(line):
            # we need to find the first occurence of 'mul(' 
            # within the `line` string starting from last_idx
            idx = line.find('mul(', last_idx)
            # if we can't find 'mul(' in the line, we break the while loop
            if idx == -1:
                break
            # we need to find the first occurence of ')' after the 'mul('
            # we found in the previous step the start of the substring
            start_idx = idx
            # we need to find the end of the substring which is the first occurence of ')'
            end_idx = line.find(')', start_idx+1)
            # if we can't find ')' in the line, we break the while loop
            if end_idx == -1:
                break
            
            # we extract the substring between the start and end index
            # and check if it has the format 'mul(x,y)' where x and y are integers
            factors = line[start_idx+4:end_idx].split(',')
            
            # if the substring has the correct format, we multiply the two factors
            if len(factors) == 2 and factors[0].isdigit() and factors[1].isdigit():
                res += int(factors[0]) * int(factors[1])
                # we update the last_idx to the end of the valid mul(x,y) substring
                last_idx = end_idx + 1
            else:
                last_idx += 1
            
            
    return res



def part2(data: list[str]) -> int:
    res = 0
    actions = []
    
    line = ''.join(data)
    dont_idx = line.find("don't()")
    last_idx = 0
    while last_idx != -1 and last_idx < len(line):
        # check for enabled or disabled commands
        idx = line.find('mul(', last_idx)
        if idx == -1:
            break
        start_idx = idx
        end_idx = line.find(')', start_idx+1)
        if dont_idx < start_idx and dont_idx != -1:
            print('disabled', dont_idx, start_idx)
            do_idx = line.find('do()', last_idx)
            if do_idx == -1:
                break

            last_idx = do_idx + 1
            dont_idx = line.find("don't()", last_idx)
            continue
        if end_idx == -1:
            break
        factors = line[start_idx+4:end_idx].split(',')
        if len(factors) == 2 and factors[0].isdigit() and factors[1].isdigit():
            res += int(factors[0]) * int(factors[1])
            last_idx = end_idx + 1
        else:
            last_idx += 1

    return res

def main():
    test_file = 'test.txt'

    input_file = 'input.txt'

    print("---- Part 1 ----")
    test_input = read_input(test_file)
    test_output = part1(test_input)
    part1_expected_test_output = 161
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
    part2_expected_test_output = 48
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