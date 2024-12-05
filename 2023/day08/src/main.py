import math

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
    data = ('\n'.join(data)).split('\n\n')
    directions = data[0]
    network = {}
    for line in data[1].split('\n'):
        # each line has a format of `element = (left, right)`
        element, sides = line.split(' = ')
        left, right = sides[1:-1].split(', ')
        network[element] = (left, right)
    
    def get_destination(element, dir):
        left, right = network[element]
        if dir == 'L':
            return left
        return right
    cur_element = 'AAA'
    steps = 0
    while cur_element != 'ZZZ':
        cur_element = get_destination(cur_element, directions[steps%len(directions)])
        steps+=1

    return steps

def part2(data):
    data = ('\n'.join(data)).split('\n\n')
    directions: str = data[0]
    network: dict[str, (str, str)] = {}
    cur_nodes: list[str] = []
    end_nodes: list[str] = []
    for line in data[1].split('\n'):
        # each line has a format of `element = (left, right)`
        element, sides = line.split(' = ')
        left, right = sides[1:-1].split(', ')
        network[element] = (left, right)
        if element.endswith('A'):
            cur_nodes.append(element)
        if element.endswith('Z'):
            end_nodes.append(element)
    
    def get_destination(element: str, dir: str) -> str:
        left, right = network[element]
        if dir == 'L':
            return left
        return right
    
    steps: int = 0

    mem = {}
    while True:
        # if all cur_nodes ends with 'Z', then we are done
        if all([node[-1] == 'Z' for node in cur_nodes]):
            break
        
        next_nodes = []
        for node in cur_nodes:
            dir = directions[steps%len(directions)]
            next_node = get_destination(node, dir)
            if next_node.endswith('Z'):
                mem[node] = steps+1
                continue
                    
            if next_node not in next_nodes:
                next_nodes.append(next_node)
            
        cur_nodes = next_nodes
        steps+=1

    # mem[key] for key in mem.keys()
    # find the least common multiple of all the values in mem
    return math.lcm(*mem.values())

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
    
    output = part1(input)
    print('Part 1 Output:', output)

    # part 2
    print("---- Part 2 ----")
    test_file_2 =  'test2.txt'
    test_input = read_input(test_file_2)
    part2_expected_test_output = 6
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