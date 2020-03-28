import sys
import math
import random


class Coordinate:
    def __init__(self, x, y):
        self._x = x
        self._y = y

    def __str__(self):
        return f"({self._x}, {self._y})"
        
    def __repr__(self):
        return f"({self._x}, {self._y})"
    
    def act(self, ac):
        if ac == 'N':
            return Coordinate(self._x, self._y - 1)
        elif ac == 'S':
            return Coordinate(self._x, self._y + 1)
        elif ac == 'W':
            return Coordinate(self._x - 1, self._y)
        else:
            return Coordinate(self._x + 1, self._y)
        
class Grid:
    def __init__(self, width, height, grid_l):
        self._width = width
        self._height = height
        self._grid = grid_l


    def valid_co(self, co):
        return not (co._x < 0 or co._x >= self._width or co._y < 0 or co._y >= self._height)
    
    def get_e(self, co):
        if not self.valid_co(co):
            raise ValueError(f"{co} outside range")
        
        return self._grid[co._y][co._x]
        

class AdvAction:
    def __init__(self, grid):
        self._path = []
        self._inv_path = []
        self._grid = grid

    def add_act(self, act):
        self._path.append(act)

        if act == 'N':
            self._inv_path.insert(0, 'S')
        elif act == 'S':
            self._inv_path.insert(0, 'N')
        elif act == 'E':
            self._inv_path.insert(0, 'W')
        else:
            self._inv_path.insert(0, 'E')

    def process_adv_action(self, act):
        #l_a = []
        a = act.split('|')
        for i in a:
            e = i.split(' ')
            if e[0] == 'MOVE':
                self.add_act(e[1])
                print(f"added action {e[1]}", file=sys.stderr)
                lv = self.check_all_grid(self._inv_path)
                print(len(lv), file=sys.stderr)
                if len(lv) < 10:
                    print("****************", lv, file=sys.stderr)
            elif e[0] == 'SURFACE':
                pass
                #l_a.append(['S', int(e[1])])
            else:
                pass
                #l_a.append(['T',int(e[1]),int(e[2])])
            
        #return l_a

    def check_path(self, path, co_st):
        co_cur = co_st
        
        if self._grid.get_e(co_cur) == 'x':
            return False
        
        for p in path:
            co_cur = co_cur.act(p)
            #print(co_cur, file=sys.stderr)
            if not self._grid.valid_co(co_cur) or self._grid.get_e(co_cur) == 'x':
                return False
            
        return True

    def check_all_grid(self, path):
        ret_co = []
        for x in range(self._grid._width-1):
            for y in range(self._grid._height-1):
                if self.check_path(path, Coordinate(x, y)):
                    ret_co.append(Coordinate(x, y))
        #print(self.check_path(path, Coordinate(0, 0)), file=sys.stderr)
        return ret_co
                    





    
    
#def create_obj(l_a, x, y):
#    for e in l_a:
#        if e[0] == 'S':
            
def choose_dir(x, y, grid):
    
    
    c = ['N','S','E','W']
    #random.shuffle(c)
    
    m_w = len(grid[0]) - 1
    m_h = len(grid) - 1
    
    print(x,y, file=sys.stderr)
    print(m_w, m_h, file=sys.stderr)
    for i in c:
        if i=='W' and x>0 and grid[y][x-1] == '.': return i
        if i=='E' and x < m_w and grid[y][x+1] == '.': return i
        if i=='S' and y < m_h and grid[y+1][x]== '.': return i
        if i=='N' and y> 0 and grid[y-1][x] == '.': return i
        
    print("NO POSS", file=sys.stderr) 
    return None
# Auto-generated code below aims at helping you parse
# the standard input according to the problem statement.
grid = []
width, height, my_id = [int(i) for i in input().split()]
for i in range(height):
    line = input()
    grid.append(list(line))
# Write an action using print
# To debug: print("Debug messages...", file=sys.stderr)

print("Debug messages...", grid, file=sys.stderr)

grd = Grid(width, height, grid)

wr=0 # random.randint(0, width-1)
hr= height-1 # random.randint(0, height-1)

while grid[hr][wr] != '.':
    #wr=random.randint(0, width-1)
    hr=hr-1 # random.randint(0, height-1)
    
print(f"{wr} {hr}")
# game loop

adv = AdvAction(grd)

while True:
    x, y, my_life, opp_life, torpedo_cooldown, sonar_cooldown, silence_cooldown, mine_cooldown = [int(i) for i in input().split()]
    sonar_result = input()
    opponent_orders = input()

    # Write an action using print
    # To debug: print("Debug messages...", file=sys.stderr)
    print(sonar_result, file=sys.stderr)
    print(opponent_orders, file=sys.stderr)
    adv.process_adv_action(opponent_orders)
    
    grid[y][x]='o'  
    
    di = choose_dir(x, y, grid)
    if di is None:
        print("SURFACE")
        for i in range(height):
            for j in range(width):
                if grid[i][j] == 'o':
                    grid[i][j] = '.'
    else:
        print(f"MOVE {di} TORPEDO")
