
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
    data: list[str] = list(data[0])
    
    disk = []
    dcount = 0
    queue = []
    for i in range(len(data)):
        n = int(data[i])
        if i % 2 == 0:
            disk += [str(int(i//2)) for _ in range(int(data[i]))]
        else:
            dcount+=n
            disk += ['.' for _ in range(n)]
    
    disk = list(disk)
    i = len(disk)
    
    while not all([x == '.' for x in disk[-dcount:]]) and i > 0:
        i-=1
        
        if disk[i] == '.':
            continue
        replace_i = disk.index('.')
        
        disk[replace_i] = disk[i]
        disk[i] = '.'
    
    ans = 0
    for i in range(len(disk)):
        if disk[i] != '.':
            ans += i*int(disk[i])
        else: 
            break
    return ans

def part2_convert_data(data):
    pass

def part2(data):
    data: list[str] = list(data[0])
    
    disk = []
    dcount = 0
    queue = []
    for i in range(len(data)):
        n = int(data[i])
        if i % 2 == 0:
            disk += [str(int(i//2)) for _ in range(int(data[i]))]
        else:
            dcount+=n
            disk += ['.' for _ in range(n)]
    
    disk = list(disk)
    i = len(disk)
    
    # while not all([x == '.' for x in disk[-dcount:]]) and i > 0:
    #     i-=1
        
    #     if disk[i] == '.':
    #         continue
    #     replace_i = disk.index('.')
        
    #     disk[replace_i] = disk[i]
    #     disk[i] = '.'
    
    cur_n = len(data)//2
    while cur_n > 0:
        i = disk.index(str(cur_n))
        repeat = int(data[cur_n*2])
        # print(i, cur_n, data[cur_n*2])
        dindex = disk.index('.')
        while dindex < len(disk):
            space = 0

            while dindex < len(disk) and disk[dindex] == '.':
                space+=1
                dindex+=1
            # print('check space', space, 'repeat', repeat, space >= repeat)
            if space >= repeat:
                # move cur_n
                # print('move', cur_n)
                local_dot_index = dindex - space
                i += (repeat - 1)
                if local_dot_index < i:
                    for _ in range(repeat):
                        disk[local_dot_index] = cur_n
                        disk[i] = '.'
                        i-=1
                        local_dot_index+=1
                break
            while dindex < len(disk) and disk[dindex] != '.':
                dindex+=1
        
        
        cur_n-=1
        # print(''.join([str(x) for x in disk]))
    ans = 0
    for i in range(len(disk)):
        if disk[i] != '.':
            ans += i*int(disk[i])
        
    return ans


test_file = 'test.txt'

input_file = 'input.txt'

# print("---- Part 1 ----")
test_input = read_input(test_file)
# test_output = part1(test_input)
# part1_expected_test_output = 1928
# if test_output == part1_expected_test_output:
#     print('Test passed')
#     input = read_input(input_file)
#     output = part1(input)
#     print('Part 1 Output:', output)
# else:
#     print('Test failed')
#     print('Expected:', part1_expected_test_output)
#     print('Got:', test_output)
    
# part 2
print("---- Part 2 ----")
part2_expected_test_output = 2858
test_output = part2(test_input)
if test_output == part2_expected_test_output:
    print('Test passed')
    input = read_input(input_file)
    output = part2(input)
    print('Part 2 Output:', output)
else:
    print('Test failed')
    print('Expected:', part2_expected_test_output)
    print('Got:', test_output)
