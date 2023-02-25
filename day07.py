from input import input
from collections.abc import Callable

class TreeNode:
    def __init__(self, name, is_leaf):
        self.name = name
        self.parent = None
        self.size = 0
        self.children = []
        self.is_leaf = is_leaf

    def add_child(self, obj):
        self.children.append(obj)

    def def_parent(self, obj):
        self.parent = obj

    def set_size(self, size):
        self.size = size


def read_input() -> TreeNode:

    root = TreeNode('', is_leaf=False)
    curr = root
    curr.def_parent(None)
    only_child = TreeNode('/', is_leaf=False)
    curr.add_child(only_child)
    only_child.def_parent(curr)

    
    for line in input():
        # on cd we change curr
        if line.startswith("$ cd") :
            tokens = line.split(' ')

            if tokens[2] == '..':
                curr = curr.parent
            else:
                curr = [child for child in curr.children if child.name == tokens[2]][0]
                
        elif line.startswith("$ ls") :
            continue

        # we add children
        else:
            tokens = line.split(' ')
            is_leaf = False if tokens[0] == 'dir' else True
            child = TreeNode(tokens[1], is_leaf)
            curr.add_child(child)
            child.def_parent(curr)
            if is_leaf:
                child.set_size(int(tokens[0]))

    return root


# given
root = read_input()

# this function mutates node object
def calculate_sizes(node: TreeNode):
    if node.is_leaf:
        return node.size
    size = sum([calculate_sizes(child) for child in node.children])
    node.set_size(size)
    return size

calculate_sizes(root)

# part 1
# this function mutates size_list
def find_nodes(node: TreeNode, size_list: list[int], validator: Callable[[TreeNode], bool]) -> None:
    if node.is_leaf:
        return 

    if validator(node):
        size_list.append((node.name, node.size))

    for child in node.children:
        find_nodes(child, size_list, validator)
     

size_list_100k: list[int]= []
find_nodes(root, size_list_100k, lambda node: True if node.size <= 100000 else False)
print(sum([size for _, size in size_list_100k])) # 1778099

# part 2
need = 30000000 - (70000000 - root.children[0].size) 
size_list = []
find_nodes(root, size_list, lambda _: True )
mapped = sorted([size for _, size in size_list])
print([num for num in mapped if num > need][0]) #1623571

