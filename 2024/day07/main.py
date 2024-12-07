
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
    import re
    converted = []
    for line in data:
        result = re.search(r'(\d+):(( \d+)+)', line)
        
        expected_result =int (result.group(1))
        elements = [
            int(element.strip()) 
            for element in result.group(2).strip().split(" ")
        ]

        converted.append((expected_result, elements))
    return converted

def part1(data):
    data = convert_data(data)
    res = 0
    def recurring_total(nodes):
        totals = []
        if len(nodes) == 2:
            totals = [nodes[0] + nodes[1], nodes[0] * nodes[1]]
        else:
            cur_node = nodes[0]
            totals = [(cur_node + total) for total in recurring_total(nodes[1:])]  + [(cur_node * total) for total in recurring_total(nodes[1:])]
        
        return totals

    for (expected_result, elements) in data:
        possible_results = recurring_total(elements[::-1])
        
        if expected_result in possible_results:
            res += expected_result    
        
    return res

def part2_convert_data(data):
    pass

def part2(data):
    import math
    data = convert_data(data)
    res = 0

    for (expected_result, elements) in data:
        def recurring_total(nodes):
            totals = []
            if len(nodes) == 2:
                totals = [x for x in [nodes[0] + nodes[1], nodes[0] * nodes[1], nodes[0] + nodes[1]*math.pow(10,len(str(nodes[0])))]]
                # print(totals)
            else:
                cur_node = nodes[0]
                totals = [(cur_node + total) for total in recurring_total(nodes[1:])] + [(cur_node * total) for total in recurring_total(nodes[1:])] + [(total*math.pow(10,len(str(cur_node))) + cur_node) for total in recurring_total(nodes[1:])]
            
            return [x for x in totals if x <= expected_result]
        possible_results = recurring_total(elements[::-1])
        # print(expected_result,possible_results)
        if expected_result in possible_results:
            res += expected_result
        
    return res
    

def main():
    test_file = 'test.txt'

    input_file = 'input.txt'

    print("---- Part 1 ----")
    test_input = read_input(test_file)
    test_output = part1(test_input)
    part1_expected_test_output = 3749
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
    part2_expected_test_output = 11387
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