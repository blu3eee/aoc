
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
    def check_if_valid(word1: list[tuple[int, int]], word2: list[tuple[int, int]]):
        # we need to check if the coords are within the bounds of the data
        # and each word will form a valid word 'MAS' or 'SAM' when combined
        for coord in word1 + word2:
            if coord[0] < 0 or coord[0] >= len(data) or coord[1] < 0 or coord[1] >= len(data[0]):
                return False
        
        word1 = ''.join([data[coord[0]][coord[1]] for coord in word1])
        word2 = ''.join([data[coord[0]][coord[1]] for coord in word2])
        if (word1 == 'MAS' or word1 == 'SAM') and (word2 == 'SAM' or word2 == 'MAS'):
            return True       

        return False

    for i in range(1, len(data)-1):
        for j in range(1, len(data[i])-1):
            c = data[i][j]
            if c == 'A':
                word1 = [(i-1, j-1), (i, j), (i+1, j+1)]
                word2 = [(i+1, j-1), (i, j), (i-1, j+1)]
                if check_if_valid(word1, word2):
                    res += 1
            
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
    part2_expected_test_output = 9
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