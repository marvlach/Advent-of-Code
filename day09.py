from input import input
from enum import Enum



class Knot:
    def __init__(self, x: int, y: int):
        self.x = x
        self.y = y
        
    def _move_one(self, dir: str):
        if dir == 'R':
            self.y += 1
        elif dir == 'L':
            self.y -= 1
        elif dir == 'D':
            self.x += 1
        else:
            self.x -= 1

    def is_neighbour(self, knot: 'Knot'):
        return abs(self.x - knot.x) <= 1 and abs(self.y - knot.y) <= 1

    def get_position(self):
        return self.x, self.y

    def follow(self, knot: 'Knot'):
        if self.is_neighbour(knot):
            return
        
        if self.x == knot.x:
            self._move_one('R' if self.y < knot.y else 'L')
            return
        
        if self.y == knot.y:
            self._move_one('D' if self.x < knot.x else 'U')
            return
        
        self._move_one('D' if self.x < knot.x else 'U')
        self._move_one('R' if self.y < knot.y else 'L')
    

def read_input(num_of_knots: int):

    knots = [Knot(0, 0) for _ in range(num_of_knots)]
    head = 0
    tail = num_of_knots - 1
    tail_position_set = set()
    for line in input():
        [dir, steps] = line.split()

        for _ in range(int(steps)):
            knots[head]._move_one(dir)
            for k in range(1, num_of_knots):
                knots[k].follow(knots[k-1])
            tail_position_set.add(knots[tail].get_position())

    return tail_position_set



# part 1
print(len(read_input(2))) # 6354

# paRT 2
print(len(read_input(10))) # 2651

