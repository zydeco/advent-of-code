package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"slices"
)

var verbose bool = os.Getenv("VERBOSE") == "1"

func vprintf(format string, a ...any) {
	if verbose {
		fmt.Printf(format, a...)
	}
}

type coord struct {
	x, y int
}

type tile int

const (
	Empty tile = iota
	Wall
)

type board struct {
	width, height int
	walls         map[coord]bool
	start, finish coord
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

type reindeer struct {
	pos    coord
	facing direction
}

func readInput() board {
	scanner := bufio.NewScanner(os.Stdin)
	board := board{walls: make(map[coord]bool)}

	// read board
	y := 0
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			break
		}
		if board.width == 0 {
			board.width = len(line)
		}

		for x, z := range line {
			xy := coord{x: x, y: y}
			switch z {
			case '#':
				board.walls[xy] = true
			case 'S':
				board.start = xy
			case 'E':
				board.finish = xy
			}
		}
		board.height += 1
		y += 1
	}
	return board
}

func (d direction) back() direction {
	switch d {
	case North:
		return South
	case South:
		return North
	case West:
		return East
	case East:
		return West
	}
	panic("invalid direction")
}

func (d direction) turnClockwise() direction {
	switch d {
	case North:
		return East
	case East:
		return South
	case South:
		return West
	case West:
		return North
	}
	panic("invalid direction")
}

func (d direction) turnCounterClockwise() direction {
	switch d {
	case North:
		return West
	case West:
		return South
	case South:
		return East
	case East:
		return North
	}
	panic("invalid direction")
}

func (d direction) String() string {
	switch d {
	case North:
		return "^"
	case East:
		return ">"
	case South:
		return "v"
	case West:
		return "<"
	}
	return "?"
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

func (b *board) isFree(c coord) bool {
	// everything outside the board is walls
	return b.isInBoard(c) && !b.walls[c]
}

func (b *board) isInBoard(c coord) bool {
	return c.x >= 0 && c.y >= 0 && c.x < b.width && c.y < b.height
}

func (b board) String() string {
	s := " "
	for x := 0; x < b.width; x++ {
		s += fmt.Sprintf("%d", x%10)
	}
	s += "\n"
	for y := 0; y < b.height; y++ {
		s += fmt.Sprintf("%d", y%10)
		for x := 0; x < b.width; x++ {
			xy := coord{x: x, y: y}
			if xy == b.start {
				s += "S"
			} else if xy == b.finish {
				s += "E"
			} else if !b.isFree(xy) {
				s += "#"
			} else {
				s += "."
			}
		}
		s += "\n"
	}
	return s
}

type visit struct {
	cost   int
	facing direction
}

func maybeVisit(b *board, visited *map[coord]visit, candidates *[]coord, cheapest *int, from coord, cost int, facing direction) {
	next := from.next(facing)
	if b.isFree(next) && cost <= *cheapest && ((*visited)[next].cost == 0 || (*visited)[next].cost > cost) {
		(*visited)[next] = visit{cost: cost, facing: facing}
		*candidates = append(*candidates, next)
		if next == b.finish && cost < *cheapest {
			*cheapest = cost
		}
	}
}

func part1(b board) {
	visited := make(map[coord]visit)
	visited[b.start] = visit{cost: 0, facing: East}
	cheapest := math.MaxInt
	candidates := make([]coord, 0, 10)
	candidates = append(candidates, b.start)
	for len(candidates) > 0 {
		var pos coord
		pos, candidates = candidates[0], candidates[1:]
		cost := visited[pos].cost
		facing := visited[pos].facing

		maybeVisit(&b, &visited, &candidates, &cheapest, pos, cost+1, facing)
		maybeVisit(&b, &visited, &candidates, &cheapest, pos, cost+1001, facing.turnCounterClockwise())
		maybeVisit(&b, &visited, &candidates, &cheapest, pos, cost+1001, facing.turnClockwise())

		// sort candidates
		slices.SortFunc(candidates, func(a, b coord) int {
			return visited[b].cost - visited[a].cost
		})
	}

	result := visited[b.finish].cost
	fmt.Println("Part 1:", result)
}

func main() {
	board := readInput()
	fmt.Printf("Input:\n%s\n", board)
	part1(board)
}
