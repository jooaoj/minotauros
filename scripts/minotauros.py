#!/usr/bin/env python3
# A BFS (Breadth-First Search) algorithm 
# for solving a maze given as a command line argument.
# git@github.com:jooaoj
import os
import sys
import time
from collections import deque
import timeit

def parseMaze(mazeSource):
	with open(mazeSource, 'r') as file:
		return [list(line.strip()) for line in file.readlines()]

def display(grid, route):
	if not route:
		return

	solution = [row[:] for row in grid]
	# Mark the solution skipping start and end
	for i, j in route[1:-1]:
		solution[i][j] = '*'

	for row in solution:
		print("\t", end='')
		print(''.join(row))

def routing(maze, moveLimit):
	start = None
	for i in range(len(maze)):
		for j in range(len(maze[i])):
			if maze[i][j] == '^':
				start = (i, j)
				break
		if start:
			break
	if not start:
		return None

	directions = [ (-1, 0), (1, 0), (0, -1), (0, 1) ]
	queue = deque( [ (start, [start], 0) ] )
	visited = set([start])

	while queue:
		current, path, moves = queue.popleft()

		if maze[current[0]][current[1]] == 'E':
			return path
		else:
			display(maze, path)
			#time.sleep(0.01) # 1000 ms
			os.system('clear')

		if moves >= moveLimit:
			continue

		for direction in directions:
			next = (current[0] + direction[0], current[1] + direction[1])

			# 0 <= next < height and 
			# 0 <= next < width and
			# not wall and
			# not visited
			if (0 <= next[0] < len(maze) and
				0 <= next[1] < len(maze[0]) and
				maze[next[0]][next[1]] != '#' and
				next not in visited):
				
				visited.add(next)
				queue.append((next, path + [next], moves + 1))

	return None 

def main():
	try:
		maze = parseMaze(sys.argv[1])
	except IndexError:
		print("minotauros: missing arguments")
		print("Try 'path/to/minotauros.py path/to/your/maze'")
		return 1

	limit = [20, 150, 200]
	complexity = []

	for test in limit:
		print(f"\n\tTrying with {test} moves:")

		start = time.time()
		pathing = routing(maze, test)
		end = time.time()
		complexity.append(round(end - start, 2)) # s

		display(maze, pathing)
		
		if not pathing:
			print("\tNo route found\n")
		else:
			length = len(pathing)

	print(f"\n\n\tFound exit in {length} moves in {max(complexity)} s!")

	return 0

def test():
    """Stupid test function"""
    L = [i for i in range(100)]

if __name__ == "__main__":
	main()
