
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

class File:
    def __init__(self, parent, name, size):
        self.name = name
        self.parent = parent
        self.size = size

class Folder:
    def __init__(self, parent, name):
        self.folders: dict[str, Folder] = {}
        self.files: list[File] = []
        self.name = name
        self.parent = parent

    def getSize(self) -> int:
        ans = sum([f.size for f in self.files])
        for folder_name in self.folders.keys():
            ans+=self.folders[folder_name].getSize()
        return ans

def convert_data(data: list[str]):
    cur_node = Folder(None, '/')
    root_node = cur_node
    commands_index = [i for i in range(len(data)) if data[i].startswith("$")]
    
    for i in range(len(commands_index)):
        if i == 0:
            continue
        cmd_index = commands_index[i]
        content_end_idx = commands_index[i+1] if i != len(commands_index) - 1 else len(data)

        cmd = data[cmd_index:content_end_idx][0]
        cmd_response = data[cmd_index:content_end_idx][1:]

        cmd = cmd[2:]
        if cmd.startswith('ls'):
            dirs: dict[str, Folder] = {}
            files: list[File] = []
            for line in cmd_response:
                if line.startswith('dir'):
                    name = line.split(' ')[1]
                    dirs[name] = Folder(cur_node, name)
                else:
                    size, name = line.split(' ')
                    size = int(size)
                    files.append(File(cur_node, name, size))
                    
            cur_node.files = files
            cur_node.folders = dirs
        elif cmd.startswith('cd'):
            target = cmd.split(" ")[1]
            if target == '..':
                cur_node = cur_node.parent
            else:
                cur_node = cur_node.folders[target]

    return root_node

def part1(data):
    root_node = convert_data(data)
    
    def check_dir(dir: Folder):
        s = 0
        dir_size = dir.getSize()
        if dir_size <= 100000:
            s += dir_size                
        for subdir in dir.folders:
            s += check_dir(dir.folders[subdir])
        return s

    ans = check_dir(root_node)
    return ans

def part2(data):
    root_node = convert_data(data)
    total_space = 70000000
    using = root_node.getSize()
    print('using', using)
    print('need', total_space - using)
    space_needed = 30000000 - (total_space - using)
    folder_sizes = []
    def get_folders_size(node: Folder):
        folder_sizes.append(node.getSize())
        for dir_name in node.folders:
            dir = node.folders[dir_name]
            dir_size = dir.getSize()
            folder_sizes.append(dir_size)
            for subdir_name in dir.folders:
                get_folders_size(dir.folders[subdir_name])
        
    get_folders_size(root_node)
    
    return min([x for x in folder_sizes if x >= space_needed])
    

test_file = 'test.txt'

input_file = 'input.txt'

print("---- Part 1 ----")
test_input = read_input(test_file)
test_output = part1(test_input)
part1_expected_test_output = 95437
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
part2_expected_test_output = 24933642
test_output = part2(test_input)
if test_output == part2_expected_test_output:
    print('Test passed')
    output = part2(input)
    print('Part 2 Output:', output)
else:
    print('Test failed')
    print('Expected:', part2_expected_test_output)
    print('Got:', test_output)
