
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
    converted_data: list[list[int]] = []
    for line in data:
        line_data = []
        for n in line.split(' '):
            line_data.append(int(n))

        converted_data.append(line_data)
    return converted_data
    

def part1(data):
    converted_data = convert_data(data)
    safe_count = 0
    for line in converted_data:
        is_increase = line[0] < line[1]
        for i in range(1, len(line)):
            if (is_increase and line[i-1] > line[i]) or (not is_increase and line[i-1] < line[i]) or abs(line[i-1] - line[i]) > 3 or line[i-1] == line[i]:
                break
        else:
            safe_count += 1
    return safe_count

def is_increasing_levels(levels):
    increase_count = 0
    for x in levels:
        increase_count += 1 if x > 0 else 0
    is_increasing = increase_count > len(levels) / 2
    return is_increasing

def is_safe(levels, is_increasing):
    # check if the levels are safe
    # it has to be increasing or decreasing without a gap of more than 3 or 0
    
    for i in range(0, len(levels)):
        if (is_increasing and levels[i] < 0) or (not is_increasing and levels[i] > 0) or abs(levels[i]) > 3 or levels[i] == 0:
            return False
    
    return True

def part2(data):
    converted_data = convert_data(data)
    safe_count = 0

    safe_lines = []

    def get_levels(line: list[str]) -> list[int]:
        levels = []
        for i in range(1, len(line)):
            level = line[i] - line[i-1]
            levels.append(level)
        return levels

    for line in converted_data:
        levels = get_levels(line)
        
        is_increasing = is_increasing_levels(levels)
        
        if is_safe(levels, is_increasing):    
            safe_count += 1
            safe_lines.append(line)
            continue
        
        for i in range (len(levels)+1):
            new_line = line.copy()
            new_line.pop(i)
            new_levels = get_levels(new_line)
            
            if is_safe(new_levels, is_increasing):    
                safe_count += 1
                safe_lines.append(new_line)
                break
            
        # already_removed = False
        # # check if can be fixed
        # for i in range(len(levels)):
        #     level = levels[i]
        #     if level == 0 or abs(level) > 3 or (is_increasing and level < 0) or (not is_increasing and level > 0):
        #         if i == 0:
        #             already_removed = True
        #             continue
        #         if i == len(levels) - 1:
        #             if not already_removed:
        #                 safe_count+=1
        #             break
        #         # check if can be fixed
        #         new_levels = levels[0:i] + [levels[i]+levels[i+1]] + levels[i+2:]
        #         if is_safe(new_levels, is_increasing):
        #             print ('check if can be fixed', line, levels)
        #             print('old levels:', levels)
        #             print('new levels:', new_levels, is_increasing)
        #             print('fixed')
        #             safe_count+=1
        #             safe_lines.append(line)
        #             break        
    write_output('safe.txt', '\n'.join(' '.join([str(x) for x in line]) for line in safe_lines))
    return safe_count

def main():
    test_file = 'test.txt'

    input_file = 'input.txt'

    print("---- Part 1 ----")
    test_input = read_input(test_file)
    test_output = part1(test_input)
    part1_expected_test_output = 2
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
    part2_expected_test_output = 4
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