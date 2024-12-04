
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

word = 'XMAS'
def part1(data: list[str]) -> int:
    res = 0
    
    def check_word_puzzle(coords: list[tuple[int, int]]):
        print('Checking:', coords)
        for i in range(len(coords)):
            coord = coords[i]
            # first, need to check if the coord is within the bounds of the data
            if coord[0] < 0 or coord[0] >= len(data) or coord[1] < 0 or coord[1] >= len(data[0]) or data[coord[0]][coord[1]] != word[i]:
                return False
        return True
    
    for i in range(len(data)):
        for j in range(len(data[i])):
            c = data[i][j]
            if c == word[0]:
                # check if the word is in the puzzle in any of the 8 directions
                possible_directions = [
                    [(i, j), (i, j+1), (i, j+2), (i, j+3)],
                    [(i, j), (i+1, j), (i+2, j), (i+3, j)],
                    [(i, j), (i+1, j+1), (i+2, j+2), (i+3, j+3)],
                    [(i, j), (i+1, j-1), (i+2, j-2), (i+3, j-3)],
                    [(i, j), (i, j-1), (i, j-2), (i, j-3)],
                    [(i, j), (i-1, j), (i-2, j), (i-3, j)],
                    [(i, j), (i-1, j-1), (i-2, j-2), (i-3, j-3)],
                    [(i, j), (i-1, j+1), (i-2, j+2), (i-3, j+3)]
                ]
                for direction in possible_directions:
                    if check_word_puzzle(direction):
                        res += 1     
    
    return res

def part2_convert_data(data):
    pass

def part2(data: list[str]) -> int:
    res = 0
    
    return res

def main():
    test_file = 'test.txt'

    input_file = 'input.txt'

    print("---- Part 1 ----")
    test_input = read_input(test_file)
    test_output = part1(test_input)
    part1_expected_test_output = 18
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
    part2_expected_test_output = 0
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