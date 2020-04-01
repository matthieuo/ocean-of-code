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

    def __eq__(self, other):
        return self._x == other._x and self._y == other._y
    
    def __hash__(self):
        return hash((self._x, self._y))
    
    def dist(self, c1):
        return abs(self._x - c1._x) + abs(self._y - c1._y)
    
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
        

    def num_avail_pos(self, cur_pos):
        """ return the number of possible positions when all actions are played"""

        def rec_pos(cur_pos, hist):
            hist.add(cur_pos)
            
            if not self.valid_co(cur_pos) or self.get_e(cur_pos) != '.':
                return 0

            #if cur_pos in hist:
            #    return hist[cur_pos]
            
            sum_a = 1
            for a in ['E', 'W', 'S', 'N']:
                if cur_pos.act(a) in hist:
                    continue
                sum_a += rec_pos(cur_pos.act(a), hist)

            #hist[cur_pos] = sum_a
            return sum_a

        ret_l = []
        for a in ['E', 'W', 'S', 'N']:
            ret_l.append((a, rec_pos(cur_pos.act(a), set())))
            
        return sorted(ret_l, key=lambda tup: tup[1], reverse=True)
        

    
        
class AdvAction:
    def __init__(self, grid):
        self._path = []
        self._inv_path = []
        self._grid = grid

        # initial search set
        self._search_area = set()
        for x in range(self._grid._width):
            for y in range(self._grid._height):
                self._search_area.add(Coordinate(x, y))




    def identify_sectors_search(self):

        sector_elt = {}
        for s in range(1, 10): # 9 sectors
            sector_elt[s] = 0
            
            if s == 1:
                rx, ry = (0, 0)
            elif s == 2:
                rx, ry = (5, 0)
            elif s == 3:
                rx, ry = (10, 0)
            elif s == 4:
                rx, ry = (0, 5)
            elif s == 5:
                rx, ry = (5, 5)
            elif s == 6:
                rx, ry = (10, 5)
            elif s == 7:
                rx, ry = (0, 10)
            elif s == 8:
                rx, ry = (5, 10)
            else:
                rx, ry = (10, 10)

            for x in range(rx, rx + 5):
                for y in range(ry, ry + 5):
                    if Coordinate(x, y) in self._search_area:
                        sector_elt[s] += 1

        return sector_elt
            
    def reset_path(self):
        self._path = []
        self._inv_path = []

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
                lv = self.check_search_area(self._path)
                print(len(lv), file=sys.stderr)
                
                if len(lv)  < 50:
                    print("****************", lv, file=sys.stderr)
                if len(lv) < 4:
                    #print("MSG I know ah ah ah ! ")
                    print("****************", lv, file=sys.stderr)
                    return lv[0]
                    
            elif e[0] == 'SURFACE':
                self.new_aera_surface(self, int(e[1]))
                lv = self.check_search_area(self._path)
                print("SUR", len(lv), file=sys.stderr)
                
                if len(lv)  < 50:
                    print("**SUR", lv, file=sys.stderr)
                    
                if len(lv) < 4:
                    #print("MSG I know ah ah ah ! ")
                    print("****************", lv, file=sys.stderr)
                    return lv[0]
                #su = self.check_surface(self._inv_path,  int(e[1]))
                #print("sur", len(su), file=sys.stderr)
                #if len(su) < 10:
                #    print("sssssssss", su, file=sys.stderr)
                #l_a.append(['S', int(e[1])])
            elif e[0] == 'SILENCE':
                
                lv = self.check_search_area(self._path)
                if len(lv)  < 50:
                    print("sil len", lv, file=sys.stderr)
                self.reset_path()
                new_search_area = set()
                for c in lv:
                    for i in range(-1, 2):
                        new_search_area.add(Coordinate(min(max(c._x + i, 0), self._grid._width-1),
                                                       c._y))
                        
                        new_search_area.add(Coordinate(c._x,
                                                       min(max(c._y + i, 0), self._grid._height-1)))

                print("new, old ", len(new_search_area), len(self._search_area), file=sys.stderr)
                print(new_search_area, file=sys.stderr)
                self._search_area = new_search_area
                # add new start coord
            
        return None

    def check_path(self, path, co_st):
        co_cur = co_st
        
        if self._grid.get_e(co_cur) == 'x':
            return None
        
        for p in path:
            co_cur = co_cur.act(p)
            #print(co_cur, file=sys.stderr)
            if not self._grid.valid_co(co_cur) or self._grid.get_e(co_cur) == 'x':
                return None
            
        return co_cur

    def new_aera_surface(self, path, s):
        new_search_area = set()

        if s == 1:
            rx, ry = (0, 0)
        elif s == 2:
            rx, ry = (5, 0)
        elif s == 3:
            rx, ry = (10, 0)
        elif s == 4:
            rx, ry = (0, 5)
        elif s == 5:
            rx, ry = (5, 5)
        elif s == 6:
            rx, ry = (10, 5)
        elif s == 7:
            rx, ry = (0, 10)
        elif s == 8:
            rx, ry = (5, 10)
        elif s == 9:
            rx, ry = (10, 10)
            
        for x in range(rx, rx + 5):
            for y in range(ry, ry + 5):
                last_co = self.check_path(self._inv_path, Coordinate(x,y))
                if last_co is not None:
                    new_search_area.add(last_co)
                    
        print("SUR new, old ", len(new_search_area), len(self._search_area), file=sys.stderr)
        #print(new_search_area, file=sys.stderr)
        self._search_area = new_search_area
                
            
    def check_search_area(self, path):
        ret_co = []
        print("LEN search", len(self._search_area), file=sys.stderr)
        for co in self._search_area:
            last_co = self.check_path(path, co)
            if last_co is not None:
                ret_co.append(last_co)

        return ret_co
                    


            
def choose_dir(x, y, grid, actions):
    
    
    
    #random.shuffle(c)
    
    m_w = len(grid[0]) - 1
    m_h = len(grid) - 1
    
    print(x,y, file=sys.stderr)
    print(m_w, m_h, file=sys.stderr)
    for i in actions:
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

torpedo_p = 0
silence_p = 0

while True:
    x, y, my_life, opp_life, torpedo_cooldown, sonar_cooldown, silence_cooldown, mine_cooldown = [int(i) for i in input().split()]
    sonar_result = input()
    opponent_orders = input()

    # Write an action using print
    # To debug: print("Debug messages...", file=sys.stderr)
    print(sonar_result, file=sys.stderr)
    print(opponent_orders, file=sys.stderr)
    #print("tor col", torpedo_cooldown, sonar_cooldown,file=sys.stderr)
    
    print("sect ", adv.identify_sectors_search(), file=sys.stderr)
    grid[y][x]='o'
    
    adv_co = adv.process_adv_action(opponent_orders)

    num_po_l = grd.num_avail_pos(Coordinate(x, y))

    print("numplol", num_po_l, file=sys.stderr)

    actions = [i[0] for i in num_po_l]
    
    #actions = ['N', 'E', 'S', 'W']
    
    if adv_co is not None:
        #ok we know where is the adv_co
        if adv_co._x < x:
            actions = ['W', 'N', 'S', 'E']
        elif adv_co._x > x:
            actions = ['E', 'N', 'S', 'W']
        elif adv_co._y > y:
            actions = ['S', 'E', 'W', 'N']
        elif adv_co._y < y:
            actions = ['N', 'E', 'W', 'S']
    
    di = choose_dir(x, y, grid, actions)


    add_str = ""
    if adv_co is not None:
        if adv_co.dist(Coordinate(x, y)) <= 4 and torpedo_p >= 3:
            add_str = f"|TORPEDO {adv_co._x} {adv_co._y}"
            torpedo_p -= 3
            
    if di is None:
        print("SURFACE")
        for i in range(height):
            for j in range(width):
                if grid[i][j] == 'o':
                    grid[i][j] = '.'
    else:
        if torpedo_p < 3 or add_str != "":
            print(f"MOVE {di} TORPEDO{add_str}")
            if add_str == "":
                torpedo_p = torpedo_p + 1
        else:
            if silence_p >= 6:
                cur_co = Coordinate(x,y)
                for jj in range(5):
                    n_co = cur_co.act(di)
                    if  not (grd.valid_co(n_co) and grid[n_co._y][n_co._x] == '.'):
                        break
                    grid[cur_co._y][cur_co._x] = 'o'
                    cur_co = n_co
                    
                print(f"SILENCE {di} {jj}")
                silence_p = 0
            else:
                print(f"MOVE {di} SILENCE")
                silence_p  += 1
