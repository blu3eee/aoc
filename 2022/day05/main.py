
def read_input(file):
    # read all the lines in the file and return them as a list
    with open(file, 'r') as f:
        data = f.readlines()
    
    # strip the newline character from the end of each line
    data = [line.replace("\n", "") for line in data]
    return data

def write_output(file, data):
    with open(file, 'w') as f:
        f.write(data)
    

def convert_data(data):
    data = '\n'.join(data)
    stacks, instructions = data.split("\n\n")
    stacks = stacks.split("\n")
    
    converted_stacks = [[] for _ in range(len([c for c in stacks[-1].strip().split(" ") if c.isdecimal()]))]
    
    for crate in stacks[::-1][1:]:
        for i in range(len(converted_stacks)):
            index = i*4+1
            c = crate[index:index+1]
            if c != ' ':
                converted_stacks[i].append(c)
    
    instructions = instructions.split("\n")
    converted_instructions = []
    for instruction in instructions:
        i = instruction.split(" ")
        converted_instructions.append((int(i[1]), int(i[3]), int(i[5])))

    return converted_stacks, converted_instructions

def part1(data):
    stacks, instructions = convert_data(data)
    
    for instruction in instructions:
        a, b, c = instruction
        for i in range(a):
            stacks[c-1].append(stacks[b-1].pop())
        
    return ''.join([stack[-1] for stack in stacks])

def part2(data):
    stacks, instructions = convert_data(data)
    
    for instruction in instructions:
        a, b, c = instruction
        
        stacks[c-1] = stacks[c-1] + stacks[b-1][-a:]
        stacks[b-1] = stacks[b-1][:-a]
            
        
    return ''.join([stack[-1] for stack in stacks])
    pass

test_file = 'test.txt'

input_file = 'input.txt'

print("---- Part 1 ----")
test_input = read_input(test_file)
test_output = part1(test_input)
part1_expected_test_output = 'CMZ'
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
part2_expected_test_output = 'MCD'
test_output = part2(test_input)
if test_output == part2_expected_test_output:
    print('Test passed')
    output = part2(input)
    print('Part 2 Output:', output)
else:
    print('Test failed')
    print('Expected:', part2_expected_test_output)
    print('Got:', test_output)
