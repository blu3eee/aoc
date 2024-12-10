
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
    
def convert_data(data: list[str]):
    data: str = '\n'.join(data)
    monkeys = data.split("\n\n")
    data = {}
    for i in range(len(monkeys)):
        monkey = monkeys[i]
        lines = monkey.split("\n")
        items = [int(i) for i in lines[1].split(": ")[1].split(", ")]
        
        op, target = lines[2].split(" ")[-2], lines[2].split(" ")[-1]
        div = int(lines[3].split(" ")[-1])
        true_monkey = int(lines[4].split(" ")[-1])
        false_monkey = int(lines[5].split(" ")[-1])
        data[i] = {'items': items, 'operation': op, 'target': target, 'div': div, 'true': true_monkey, 'false': false_monkey}
    return data

def part1(data):
    monkeys = convert_data(data)
    
    ans = 0
    def execute_operation(op, t1, t2):
        if op == '*':
            return t1 * t2
        if op == '+':
            return t1 + t2
        if op == '-':
            return t1 - t2
        if op == '/':
            return t1 // t2
    for _ in range(20):
        for key in monkeys:
            monkey = monkeys[key]
            for item in monkey['items']:
                target = monkey['target']
                op = monkey['op']
                new = item
                if target == 'old':
                    new = execute_operation(op, new, new)
                else:
                    new = execute_operation(op, new, int(target))
                
                # test condition
                div = monkey['div']
                
                target_monkey = monkey['true'] if new % div == 0 else monkey['false']
                monkeys[target_monkey]['items'] = monkeys[target_monkey]['items'] + [item]
            monkeys[key]['items'] = []
        pass
    return ans

def part2_convert_data(data):
    pass

def part2(data):
    pass

test_file = 'test.txt'

input_file = 'input.txt'

print("---- Part 1 ----")
test_input = read_input(test_file)
test_output = part1(test_input)
part1_expected_test_output = 10605
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
part2_expected_test_output = 0
test_output = part2(test_input)
if test_output == part2_expected_test_output:
    print('Test passed')
    output = part2(input)
    print('Part 2 Output:', output)
else:
    print('Test failed')
    print('Expected:', part2_expected_test_output)
    print('Got:', test_output)
