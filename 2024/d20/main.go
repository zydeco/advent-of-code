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

type tile int

const (
	Empty tile = iota
	Wall
)

type board struct {
	width, height int
	tiles         [][]tile
	start, finish coord
}

type direction int

const (
	North direction = iota
	East
	South
	West
)

var (
	AllDirections = [4]direction{North, East, South, West}
)

func readInput() board {
	scanner := bufio.NewScanner(os.Stdin)
	board := board{tiles: make([][]tile, 0, 10)}

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

		boardLine := make([]tile, len(line))
		for x, z := range line {
			xy := coord{x: x, y: y}
			boardLine[x] = Empty
			switch z {
			case '#':
				boardLine[x] = Wall
			case 'S':
				board.start = xy
			case 'E':
				board.finish = xy
			}
		}
		board.tiles = append(board.tiles, boardLine)
		board.height += 1
		y += 1
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

func (b *board) isFree(c coord) bool {
	// everything outside the board is walls
	return b.isInBoard(c) && b.tiles[c.y][c.x] == Empty
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

func (b board) stringWithPath(path map[coord]int) string {
	s := " "
	for x := 0; x < b.width; x++ {
		s += fmt.Sprintf("%d", x%10)
	}
	s += "\n"
	for y := 0; y < b.height; y++ {
		s += fmt.Sprintf("%d", y%10)
		for x := 0; x < b.width; x++ {
			xy := coord{x: x, y: y}
			cost, hasCost := path[xy]
			if xy == b.start {
				s += "S"
			} else if xy == b.finish {
				s += "E"
			} else if !b.isFree(xy) {
				s += "#"
			} else if hasCost {
				s += fmt.Sprintf("%d", cost%10)
			} else {
				s += "."
			}
		}
		s += "\n"
	}
	return s
}

func (c coord) neighbours() [4]coord {
	return [4]coord{
		c.next(North),
		c.next(East),
		c.next(South),
		c.next(West),
	}
}

func (b board) dijkstra(from coord) map[coord]int {
	visited := make(map[coord]int)
	candidates := make([]coord, 0, 100)
	candidates = append(candidates, from)
	visited[candidates[0]] = 0
	for len(candidates) > 0 {
		var pos coord
		pos, candidates = candidates[0], candidates[1:]
		cost := visited[pos] + 1

		for _, next := range pos.neighbours() {
			_, hasVisited := visited[next]
			if hasVisited || !b.isFree(next) {
				continue
			}
			visited[next] = cost
			candidates = append(candidates, next)
		}
		// sort candidates
		slices.SortFunc(candidates, func(a, b coord) int {
			return visited[b] - visited[a]
		})
	}
	return visited
}

func part1(b board) {
	// distance from each point
	costs := b.dijkstra(b.finish)

	// go through track in order, checking behind walls
	savings := make(map[int]int)
	for pos := b.start; pos != b.finish; pos = nextInPath(costs, pos) {
		cost := costs[pos]
		for _, dir := range AllDirections {
			next := pos.next(dir)
			if !b.isFree(next) {
				// wall
				posBehindWall := next.next(dir)
				behindCost, hasBehind := costs[posBehindWall]
				if hasBehind && behindCost < cost {
					saved := cost - behindCost - 2
					savings[saved] += 1
				}
			}
		}
	}

	showResults(savings, 100)
}

func showResults(savings map[int]int, target int) {
	if target == 0 {
		maxSaving := 0
		for saving := range savings {
			maxSaving = max(maxSaving, saving)
		}
		for saving := 0; saving <= maxSaving; saving++ {
			count := savings[saving]
			if count > 0 {
				fmt.Printf("There are %d cheats that save %d picoseconds.\n", count, saving)
			}
		}
	} else {
		savingsOverTarget := 0
		for saving, count := range savings {
			if saving >= target {
				savingsOverTarget += count
			}
		}
		if savingsOverTarget == 0 {
			showResults(savings, 0)
		} else {
			fmt.Printf("Savings ≥ %d: %d\n", target, savingsOverTarget)
		}
	}
}

func nextInPath(costs map[coord]int, pos coord) coord {
	directions := []direction{North, East, South, West}
	cost := costs[pos]
	for _, dir := range directions {
		next := pos.next(dir)
		nextCost, hasNext := costs[next]
		if hasNext && nextCost == cost-1 {
			// this is the next tile in the path
			return next
		}
	}
	panic("No next")
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

func timeSaved(costs map[coord]int, from, to coord) int {
	return costs[from] - costs[to] - from.manhattanDistance(to)
}

func part2(b board) {
	// distance from each point
	costs := b.dijkstra(b.finish)
	costs[b.finish] = 0

	// go through track in order, checking for cheats
	// cheat is any destination 20 tiles away that saves time
	cheats := make(map[[2]coord]int)
	for pos := b.start; pos != b.finish; pos = nextInPath(costs, pos) {
		for y := pos.y - 20; y <= pos.y+20; y++ {
			for x := pos.x - 20; x <= pos.x+20; x++ {
				dst := coord{x: x, y: y}
				dstCost, isOnPath := costs[dst]
				if isOnPath && dstCost < costs[pos] && pos.manhattanDistance(dst) <= 20 {
					saved := timeSaved(costs, pos, dst)
					if saved > 0 {
						cheats[[...]coord{pos, dst}] = saved
					}
				}
			}
		}
	}

	// count savings
	savings := make(map[int]int)
	for _, saving := range cheats {
		savings[saving] += 1
	}

	// show results
	fmt.Println("Part 2")
	showResults(savings, 100)
	// 2275313 is too high
	// 1032325 is too low
}

func main() {
	board := readInput()
	fmt.Printf("Input:\n%s\n", board)
	part1(board)
	part2(board)
}
