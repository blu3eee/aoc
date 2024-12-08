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

directions = '^>v<'
dirs = [(-1, 0), (0, 1), (1, 0), (0, -1)]

def part1(data):    
    map = [list(line) for line in data]
    n, m = len(map), len(map[0])
    
    cx, cy = 0, 0
    dir = '^'
    for i in range(len(map)):
        for j in range(len(map[i])):
            if map[i][j] in directions:
                cx, cy = i, j
                dir = map[i][j]
                break
    
    seen = set()
    while cx in range(n) and cy in range(m):
        seen.add((cx, cy))
        nx = cx + dirs[directions.index(dir)][0]
        ny = cy + dirs[directions.index(dir)][1]
        if nx in range(n) and ny in range(m) and map[nx][ny] == '#':
            dir = directions[(directions.index(dir) + 1) % 4]
        else:
            cx, cy = nx, ny

    return len(seen)


def part2(data):
    g = [list(line) for line in data]
    n, m = len(g), len(g[0])
    
    ix, iy = 0, 0
    cd = 0
    for i in range(len(g)):
        for j in range(len(g[i])):
            if g[i][j] in directions:
                ix, iy = i, j
                break
    
    ans = 0
    for ox in range (n):
        for oy in range(m):
            if g[ox][oy] == '#' or g[ox][oy] == '^':
                continue
            g[ox][oy] = '#'
            
            seen = set()
            cd = 0
            cx, cy = ix, iy
            while cx in range(n) and cy in range(m) and (cx,cy,cd) not in seen:
                seen.add((cx,cy,cd))
                while True:
                    cdir = dirs[cd]
                    nx, ny = cx + cdir[0], cy + cdir[1]
                    if nx in range(n) and ny in range(m) and g[nx][ny] == '#':
                        cd = (cd + 1) % 4
                    else:
                        cx, cy = nx, ny
                        break
            
            if (cx, cy, cd) in seen:
                ans += 1

            g[ox][oy] = '.'  
            
    
    return ans

def main():
    test_file = 'test.txt'

    input_file = 'input.txt'

    print("---- Part 1 ----")
    test_input = read_input(test_file)
    test_output = part1(test_input)
    part1_expected_test_output = 41
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