
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

def part1(data: list[str]):
    cycles = []
    
    for line in data:
        if line == 'noop':
           cycles.append(0)
        else:
            parts= line.split(' ')
            cmd, amount = parts[0], parts[1]
            amount = int(amount)
            cycles += [0, amount]
    x = 1
    for i in range(len(cycles)):
        x = x + cycles[i]
        cycles[i] = x
    ans = 0
    
    for i in [20, 60, 100, 140, 180, 220]:
        
        ans+=(cycles[i-2]*i)
    return ans

def part2(data):
    cycles = []
    
    for line in data:
        if line == 'noop':
           cycles.append(0)
        else:
            parts= line.split(' ')
            cmd, amount = parts[0], parts[1]
            amount = int(amount)
            cycles += [0, amount]
    x = 1
    for i in range(len(cycles)):
        x = x + cycles[i]
        cycles[i] = x
    
    crt = ""
    img = []
    for i in range(len(cycles)):
        if i % 40 == 0:
            if crt != "":
                img.append(crt)
                crt = ""
        cur_x = (1 if i == 0 else cycles[i-1]) - 1
        if i == 41:
            print(i, cur_x, range(cur_x, cur_x+3), '#' if i in range(cur_x, cur_x+3) else '.')
        crt += '#' if (i%40) in range(cur_x, cur_x+3) else '.'
    img.append(crt)
    ans = '\n'.join(img)
    
    write_output('output.txt', ans)
    
    return ans

test_file = 'test.txt'

input_file = 'input.txt'

print("---- Part 1 ----")
test_input = read_input(test_file)
test_output = part1(test_input)
part1_expected_test_output = 13140
if test_output == part1_expected_test_output:
    print('Test passed')
    input = read_input(input_file)
    output = part1(input)
    print('Part 1 Output:', output)
else:
    print('Test failed')
    print('Expected:', part1_expected_test_output)
    print('Got:', test_output)
    
# part 2
print("---- Part 2 ----")
part2_expected_test_output = """##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."""
test_output = part2(test_input)
if test_output == part2_expected_test_output:
    print('Test passed')
    output = part2(input)
    print('Part 2 Output:', output)
else:
    print('Test failed')
    print('Expected:', part2_expected_test_output)
    print('Got:', test_output)
