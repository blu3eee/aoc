
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
    data = '\n'.join(data).split("\n\n")
    section1, section2 = data
    section1 = section1.split('\n')
    section2 = [line.split(',') for line in section2.split('\n')]
    page_orders = {}
    for line in section1:
        first, after = line.split('|')
        page_orders[first] = [after] if first not in page_orders else page_orders[first] + [after]   
    return page_orders, section2

def part1(data):
    page_orders, updates = convert_data(data)
    res = 0
    for update in updates:
        for i in range(len(update)-1):
            page_no = update[i]
            this_page_afters = page_orders[page_no] if page_no in page_orders else []

            if not all([after in this_page_afters for after in update[i+1:]]):
                break
            is_valid = False
            for n in update[:i]:
                if not page_no in page_orders[n]:
                    break
            else:
                is_valid = True
            if not is_valid:
                break
        else:
            res+= int(update[len(update)//2])
    
    return res

def part2(data):

    page_orders, updates = convert_data(data)

    # we simply find the updates that are not valid and correct them
    need_to_correct = []
    for update in updates:
        non_valid = True
        for i in range(len(update)-1):
            page_no = update[i]
            this_page_afters = page_orders[page_no] if page_no in page_orders else []

            if not all([after in this_page_afters for after in update[i+1:]]):
                break
            is_valid = False
            for n in update[:i]:
                if not page_no in page_orders[n]:
                    break
            else:
                is_valid = True
            if not is_valid:
                break
        else:
            non_valid = False
        if non_valid:
            need_to_correct.append(update)
    
    res = 0
    # for each update that needs to be corrected, we find the page that is in the middle of the update
    # and add it to the result
    for update in need_to_correct:
        ranks_dict: dict[int, list[str]] = {}
        for n in update:
            update_count = len([x for x in update if x != n and n in page_orders and x in page_orders[n]])
            ranks_dict[update_count] = ranks_dict[update_count] + [n] if update_count in ranks_dict else [n]

        res+=int(ranks_dict[len(update)//2][0])

    return res

def main():
    test_file = 'test.txt'

    input_file = 'input.txt'

    print("---- Part 1 ----")
    test_input = read_input(test_file)
    test_output = part1(test_input)
    part1_expected_test_output = 143
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
    part2_expected_test_output = 123
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