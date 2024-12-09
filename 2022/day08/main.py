
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
    return [list([int(x) for x in line]) for line in data]
    


def part1(data):
    data = convert_data(data)
    n, m = len(data), len(data[0])
    ans = (n+m)*2 - 4
    
    for i in range(1, n-1):
        for j in range(1, m-1):
            h = data[i][j]
            edges = [(0, 1), (0, -1), (1, 0), (-1, 0)]
            # if any([True for (di, dj) in edges if h > data[i+di][j+dj]]):
            #     ans+=1
            for (di, dj) in edges:
                visible = True
                ci, cj = i+di, j+dj
                while ci >= 0 and ci < n and cj >= 0 and cj < m:
                    if h <= data[ci][cj]:
                        visible = False
                        break
                    ci+=di
                    cj+=dj
                if visible:
                    ans+=1
                    break
            
    return ans

def part2(data):
    data = convert_data(data)
    n, m = len(data), len(data[0])
    ans = 0
    
    for i in range(1, n-1):
        for j in range(1, m-1):
            h = data[i][j]
            edges = [(0, 1), (0, -1), (1, 0), (-1, 0)]
            # if any([True for (di, dj) in edges if h > data[i+di][j+dj]]):
            #     ans+=1
            score = 1
            for (di, dj) in edges:
                edge_score = 0
                ci, cj = i+di, j+dj
                while ci >= 0 and ci < n and cj >= 0 and cj < m:
                    
                    edge_score+=1
                    if h <= data[ci][cj]:
                        break
                    ci+=di
                    cj+=dj
                score *= edge_score
            ans = max(ans, score)
            
    return ans

test_file = 'test.txt'

input_file = 'input.txt'

print("---- Part 1 ----")
test_input = read_input(test_file)
test_output = part1(test_input)
part1_expected_test_output = 21
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
part2_expected_test_output = 8
test_output = part2(test_input)
if test_output == part2_expected_test_output:
    print('Test passed')
    output = part2(input)
    print('Part 2 Output:', output)
else:
    print('Test failed')
    print('Expected:', part2_expected_test_output)
    print('Got:', test_output)
