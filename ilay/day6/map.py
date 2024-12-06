from copy import deepcopy

class Map:
    def __init__(self, map_file: str):
        self.file = map_file
        self.map = self.load_map()
        print(f"map size: {len(self.map)}, {len(self.map[0])}")
    
    def load_map(self) -> str:
        with open(self.file, 'r') as file:
            data = file.read()
            map = []
            for line in data.split('\n'):
                map_line = [cell for cell in line]
                map.append(map_line)
                
            return map

    def print(self):
        map_clone = ""
        for line in self.map:
            map_clone += "".join(line) + "\n"
        print(map_clone)

    def get_guard_pos(self):
        for i, map_line in enumerate(self.map):
            for j, value in enumerate(map_line):
                if value == '^':
                    self.x = j
                    self.y = i
                    return
    
    def rotate_counter(self):
        list_of_tuples = zip(*self.map[::-1])
        self.map = [list(elem) for elem in list_of_tuples]

    def rotate(self):
        list_of_tuples = zip(*self.map)
        self.map = [list(elem) for elem in list_of_tuples][::-1]
        self.x, self.y = self.y, len(self.map) - self.x
    
    def do_turn(self):
        current_line = self.map[self.y]
        # print(self.x)
        # print(self.y)
        
        end_x = self.x
        done = False
        while True:
            if end_x == len(current_line) or current_line[end_x] == '#':
                if end_x == len(current_line):
                    done = True
                break
            end_x += 1
        
        for i in range(self.x, end_x):
            current_line[i] = 'X'

        self.x = end_x
        self.map[self.y] = current_line
        # self.print()
        return done

    def positions_visited(self):
        return sum(cell == 'X' for row in self.map for cell in row)

    def part1(self):
        self.rotate_counter()
        self.get_guard_pos()
        while True:
            is_done = self.do_turn()
            self.rotate()
            if is_done:
                break
        return self.positions_visited()
        
    def is_cord_invalid(self, x, y):
        return self.map[y][x] != "."

    def check_variant(self):
        for i in range(300):
            if self.do_turn():
                return False
            self.rotate()
            # self.print()
        return True

    def check_map_with_obsticale(self, obs_x, obs_y):
        if self.is_cord_invalid(obs_x, obs_y):
            return False

        self.get_guard_pos()
        
        self.map[obs_y][obs_x] = '#'
        # self.print()
        return self.check_variant()

    def part2(self):
        answer = 0
        self.rotate_counter()

        map_backup = deepcopy(self.map)
        for i in range(len(self.map)):
            for j in range(len(self.map[0])):
                if self.check_map_with_obsticale(j, i):
                    answer += 1
                self.map = deepcopy(map_backup)
            print(f"checking: {i}")
            print(f"answers: {answer}")
            
        return answer

        # self.print()
        # print(self.result())
        # print(turns)
        return is_done
        pass