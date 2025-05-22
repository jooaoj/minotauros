#import

MAZE1 = 'maze-task-first.txt'
MAZE2 = 'maze-task-second.txt'

def prepMaze():
	maze = []

	print("prepMaze")
	with open(MAZE1) as fp:
		for line in fp:
			tmp = list(line)
			maze.append(tmp)
	print(maze[18][18])


def main():
	print("main")
	prepMaze()
	return 0

#if '__name__' == '__main__':
print("starting...")
main()