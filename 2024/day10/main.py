
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
    
def find_trailhead(block, count_distinct_path):
    block = [list([int(x) for x in line]) for line in block]
    ans = 0
    zeros = []
    n, m = len(block), len(block[0])
    for i in range(n):
        for j in range(m):
            x = block[i][j]
            if x == 0:
                zeros.append((i, j))
    def find_next(x, y):
        cur_n = block[x][y]
        if cur_n == 9:
            return [(x, y)]
        ans = []
        dirs = [(0, 1), (1, 0), (0, -1), (-1, 0)]
        for (dx, dy) in dirs:
            nx = (x+dx)
            ny = (y+dy)
            if nx in range(n) and ny in range(m) and block[nx][ny] == block[x][y] + 1:
                ans += find_next(nx, ny)
        return ans
    ans = 0
    for (ix, iy) in zeros:
        trailheads = set(find_next(ix, iy)) if not count_distinct_path else find_next(ix, iy)
        ans += len(trailheads)
    
    return ans

def part1(data):
    return find_trailhead(data, False)

def part2(data):
    return find_trailhead(data, True)

test_file = 'test.txt'

input_file = 'input.txt'

print("---- Part 1 ----")
test_input = read_input(test_file)
test_output = part1(test_input)
part1_expected_test_output = 36
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
part2_expected_test_output = 81
test_output = part2(test_input)
if test_output == part2_expected_test_output:
    print('Test passed')
    output = part2(input)
    print('Part 2 Output:', output)
    
else:
    print('Test failed')
    print('Expected:', part2_expected_test_output)
    print('Got:', test_output)
