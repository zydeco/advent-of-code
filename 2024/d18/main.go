package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
)

type coord struct {
	x, y int
}

type board struct {
	width, height int
	drops         map[coord]int
}

type direction int

const (
	North direction = iota
	East
	South
	West
)

type move int

const (
	Forward move = iota
	TurnClockwise
	TurnCounterClockwise
)

func readInput() board {
	scanner := bufio.NewScanner(os.Stdin)
	board := board{drops: make(map[coord]int)}

	// read board
	time := 1
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			panic("invalid input")
		}
		var c coord
		_, err := fmt.Sscanf(line, "%d,%d", &c.x, &c.y)
		if err != nil {
			panic("invalid input")
		}
		board.drops[c] = time
		time++
	}
	if len(board.drops) < 50 {
		board.width, board.height = 7, 7
	} else {
		board.width, board.height = 71, 71
	}

	return board
}

func (c coord) next(dir direction) coord {
	switch dir {
	case North:
		return coord{x: c.x, y: c.y - 1}
	case East:
		return coord{x: c.x + 1, y: c.y}
	case South:
		return coord{x: c.x, y: c.y + 1}
	case West:
		return coord{x: c.x - 1, y: c.y}
	default:
		panic("Invalid direction")
	}
}

func (b *board) topLeft() coord {
	return coord{x: 0, y: 0}
}

func (b *board) bottomRight() coord {
	return coord{x: b.width - 1, y: b.height - 1}
}
func (b *board) isFree(c coord, time int) bool {
	// everything outside the board is walls
	dropTime, hasDrop := b.drops[c]
	return b.isInBoard(c) && (!hasDrop || dropTime > time)
}

func (b *board) isInBoard(c coord) bool {
	return c.x >= 0 && c.y >= 0 && c.x < b.width && c.y < b.height
}

func (b board) StringAt(time int) string {
	s := " "
	for x := 0; x < b.width; x++ {
		s += fmt.Sprintf("%d", x%10)
	}
	s += "\n"
	for y := 0; y < b.height; y++ {
		s += fmt.Sprintf("%d", y%10)
		for x := 0; x < b.width; x++ {
			xy := coord{x: x, y: y}
			if b.isFree(xy, time) {
				s += "."
			} else {
				s += "#"
			}
		}
		s += "\n"
	}
	return s
}

func (b board) stringWithPath(time int, path []coord) string {
	s := " "
	for x := 0; x < b.width; x++ {
		s += fmt.Sprintf("%d", x%10)
	}
	s += "\n"
	pathCoords := make(map[coord]int)
	for i, c := range path {
		pathCoords[c] = i + 1
	}
	for y := 0; y < b.height; y++ {
		s += fmt.Sprintf("%d", y%10)
		for x := 0; x < b.width; x++ {
			xy := coord{x: x, y: y}
			_, hasDrop := b.drops[xy]
			_, hasPath := pathCoords[xy]
			if !hasDrop && !hasPath {
				s += "."
			} else if hasDrop && hasPath {
				s += "@"
			} else if hasPath {
				s += "O"
			} else if hasDrop {
				s += "#"
			}
		}
		s += "\n"
	}
	return s
}

func maybeVisit(b *board, visited *map[coord]int, candidates *[]coord, next coord, cost int, time int) {
	if b.isFree(next, time) && ((*visited)[next] == 0 || (*visited)[next] > cost) {
		(*visited)[next] = cost
		*candidates = append(*candidates, next)
	}
}

func absDiff(x, y int) int {
	if x < y {
		return y - x
	}
	return x - y
}

func (c coord) manhattanDistance(c2 coord) int {
	dx := absDiff(c2.x, c.x)
	dy := absDiff(c2.y, c.y)
	return dx + dy
}

func (b *board) startTime() int {
	if b.width == 7 {
		return 12
	}
	return 1024
}

func (b board) pathLengthAtTime(time int) int {
	if !b.isFree(b.bottomRight(), time) {
		return -1
	}
	visited := make(map[coord]int)
	start := b.topLeft()
	visited[start] = 1
	candidates := make([]coord, 0, 10)
	candidates = append(candidates, start)
	for len(candidates) > 0 {
		var pos coord
		pos, candidates = candidates[0], candidates[1:]
		cost := visited[pos]

		maybeVisit(&b, &visited, &candidates, pos.next(North), cost+1, time)
		maybeVisit(&b, &visited, &candidates, pos.next(South), cost+1, time)
		maybeVisit(&b, &visited, &candidates, pos.next(East), cost+1, time)
		maybeVisit(&b, &visited, &candidates, pos.next(West), cost+1, time)

		// sort candidates
		slices.SortFunc(candidates, func(c1, c2 coord) int {
			return c2.manhattanDistance(b.bottomRight()) - c1.manhattanDistance(b.bottomRight())
			//return visited[c2] - visited[c1]
		})
	}

	return visited[b.bottomRight()] - 1
}

func part1(b board) {
	fmt.Println("Part 1:", b.pathLengthAtTime(b.startTime()))
}

func (b board) dropAt(time int) (coord, bool) {
	for c, t := range b.drops {
		if t == time {
			return c, true
		}
	}
	return coord{}, false
}

func part2(b board) {
	// bisect
	start := b.startTime()
	end := len(b.drops)
	pivot := start + (end-start)/2
	for {
		p1 := b.pathLengthAtTime(pivot)
		p2 := b.pathLengthAtTime(pivot + 1)
		if p1 == -1 && p2 == -1 {
			// no paths, search sooner
			end = pivot - 1
		} else if p1 > 0 && p2 > 0 {
			// both have paths, search later
			start = pivot + 1
		} else {
			break
		}

		pivot = start + (end-start)/2
	}

	result, found := b.dropAt(pivot + 1)
	if !found {
		panic("No result found")
	}
	fmt.Println("Part 2:", result)
}

func main() {
	board := readInput()
	part1(board)
	part2(board)
}
