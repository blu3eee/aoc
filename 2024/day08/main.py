
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
    
test_file = 'test.txt'

input_file = 'input.txt'

def part1(data):
    g = [list(line) for line in data]
    n, m = len(g), len(g[0])
    
    nodes = {}
    for i in range(n):
        for j in range(m):
            c = g[i][j]
            if c != '.':
                nodes[c] = [(i, j)] if c not in nodes else nodes[c] + [(i, j)]

    chars = nodes.keys()
    antinodes = set()
    for c in chars:
        c_nodes = nodes[c]
        for node1 in c_nodes:
            for node2 in c_nodes:
                if node1 != node2:
                    di, dj = node1[0] - node2[0], node1[1] - node2[1]
                    x, y = (node1[0] + di, node1[1] + dj)
                    # check if this antinode is in the g
                    if x in range(n) and y in range(m):
                        antinodes.add((x, y))
            
            # if i == 0 or i == n-1 or j == 0 or j == m-1:
            #     antinodes.add(c)
    return len(antinodes)


def part2(data):
    part_2_result = """##....#....#
.#.#....0...
..#.#0....#.
..##...0....
....0....#..
.#...#A....#
...#..#.....
#....#.#....
..#.....A...
....#....A..
.#........#.
...#......##"""
    part_2_result = [list(line) for line in part_2_result.split('\n')]
    g = [list(line) for line in data]
    n, m = len(g), len(g[0])
    
    nodes = {}
    for i in range(n):
        for j in range(m):
            c = g[i][j]
            if c != '.':
                nodes[c] = [(i, j)] if c not in nodes else nodes[c] + [(i, j)]

    chars = nodes.keys()
    antinodes = set()
    print(n, m)
    for c in chars:
        print('Char:', c)
        c_nodes = nodes[c]
        i, j = 0, 0
        while i < len(c_nodes):
            j=0
            while j < len(c_nodes):
                cur_node1 = c_nodes[i]
                cur_node2 = c_nodes[j]
                if cur_node1 != cur_node2:
                    while True:
                        
                        di, dj = cur_node2[0] - cur_node1[0], cur_node2[1] - cur_node1[1]
                        x, y = (cur_node1[0] - di, cur_node1[1] - dj)
                        # check if this antinode is in the g
                        if x in range(n) and y in range(m):
                            antinodes.add((x, y))
                            cur_node2 = cur_node1
                            cur_node1 = (x, y)                    
                        else:
                            
                            break
                j+=1
            i+=1
            
    g2 = g.copy()
    for i in range(n):
        for j in range(m):
            if (i, j) in antinodes:
                g2[i][j] = '#'
    new_g = '\n'.join([''.join(line) for line in g2])
    
    return n*m - new_g.count('.')

# part 1
print("---- Part 1 ----")
test_input = read_input(test_file)
test_output = part1(test_input)
part1_expected_test_output = 14
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
part2_expected_test_output = 34
test_output = part2(test_input)
if test_output == part2_expected_test_output:
    print('Test passed')
    output = part2(input)
    print('Part 2 Output:', output)
else:
    print('Test failed')
    print('Expected:', part2_expected_test_output)
    print('Got:', test_output)