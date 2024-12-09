
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
    return [(line.split(" ")[0], int(line.split(" ")[1])) for line in data]

directions = 'RDLU'
dirs = [(0, 1), (-1, 0), (0, -1), (1, 0)]
def part1(data):
    data = convert_data(data)
    head, tail = (0, 0), (0, 0)

    stepped = set([tail])
    for (dir, step) in data:
        (dx, dy) = dirs[directions.index(dir)]
        for _ in range(step):
            # move head
            hx, hy = head
            head = (hx+dx, hy+dy)
            # check and move tail
            (nhx, nhy) = head
            (tx, ty) = tail
            if abs(nhx - tx) > 1 or abs(nhy - ty) > 1:
                tail = (hx, hy)
            stepped.add(tail)        

    return len(stepped)

def part2(data):
    data = convert_data(data)
    snake: list[tuple[int, int]] = [(0, 0) for _ in range(10)]

    stepped = set([(0, 0)])
    def print_snake():
        n = max([abs(x) for (x, _) in snake]) + 1
        m = max([abs(y) for (_, y) in snake]) + 1
        
        g = [['.' for _ in range(m*2)] for _ in range(n*2)]
        g[n][m] = 's'
        for i in range(len(snake)):
            (x, y) = snake[i]
            x = x+n
            y = y+m
            if i == 0:
                g[x][y] = 'H'
            else:
                if snake[i] != snake[i-1]:
                    g[x][y] = str(i)
        print('\n'.join([''.join(line) for line in g[::-1]]))
    for index in range(len(data)):
        
        (dir, step) = data[index]
        
        for n in range(step):
            (dx, dy) = dirs[directions.index(dir)]
            # move head
            # print('move head', n)
            hx, hy = snake[0]
            snake[0] = (hx+dx, hy+dy)
            # move the tails
            for i in range (1, len(snake)):
                (hx, hy) = snake[i-1]
                (tx, ty) = snake[i]
                dx = hx - tx
                dy = hy - ty
                # print(snake[i-1], snake[i], dx, dy)
                if abs(dx) <= 1 and abs(dy) <= 1:
                    # print("not move tail", i)
                    break
                # print('move tail', i)
                # check for diagnal move
                if abs(dy) >= 1:
                    ty += 1 if dy > 0 else -1
                    # if dx != 0:
                    #     tx += 1 if dx > 0 else -1

                if abs(dx) >= 1:
                    tx += 1 if dx > 0 else -1
                snake[i] = (tx, ty)
                
            # # check and move tail
            # (nhx, nhy) = head
            # (tx, ty) = tail
            # if abs(nhx - tx) > 1 or abs(nhy - ty) > 1:
            #     tail = (hx, hy)
            stepped.add(snake[9])
            # print()
        # print(dir, step, snake)
        # print_snake()
    return len(stepped)

test_file = 'test.txt'

input_file = 'input.txt'

print("---- Part 1 ----")
test_input = read_input(test_file)
test_output = part1(test_input)
part1_expected_test_output = 13
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
test_input = """R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20""".split("\n")
part2_expected_test_output = 36
test_output = part2(test_input)
if test_output == part2_expected_test_output:
    print('Test passed')
    output = part2(input)
    print('Part 2 Output:', output)
else:
    print('Test failed')
    print('Expected:', part2_expected_test_output)
    print('Got:', test_output)
