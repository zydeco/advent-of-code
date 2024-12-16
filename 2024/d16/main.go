package main

import (
	"bufio"
	"fmt"
	"math"
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

type move int

const (
	Forward move = iota
	TurnClockwise
	TurnCounterClockwise
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

func (b board) stringWithPath(path map[coord]bool) string {
	s := " "
	for x := 0; x < b.width; x++ {
		s += fmt.Sprintf("%d", x%10)
	}
	s += "\n"
	for y := 0; y < b.height; y++ {
		s += fmt.Sprintf("%d", y%10)
		for x := 0; x < b.width; x++ {
			xy := coord{x: x, y: y}
			if path[xy] {
				s += "O"
			} else if xy == b.start {
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

func part1(b board) int {
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
	return result
}

func maybeVisit2(b *board, visited []int, candidates *[]position, cheapest *int, nextCoord coord, cost int, facing direction) {
	next := position{
		x:      nextCoord.x,
		y:      nextCoord.y,
		facing: facing,
	}
	currentCost := visited[b.posOrd(next)]
	hasCost := currentCost < math.MaxInt
	if b.isFree(nextCoord) && cost <= *cheapest && (!hasCost || cost <= currentCost) {
		visited[b.posOrd(next)] = cost
		*candidates = append(*candidates, next)
		if nextCoord == b.finish && cost < *cheapest {
			if *cheapest == math.MaxInt {
				fmt.Printf("Found first cheapest: %d\n", cost)
			}
			*cheapest = cost
		}
	}
}

type position struct {
	x, y   int
	facing direction
}

func (c coord) facing(facing direction) position {
	return position{x: c.x, y: c.y, facing: facing}
}

func (p position) coord() coord {
	return coord{x: p.x, y: p.y}
}

func (b board) posOrd(p position) int {
	return int(p.facing) + (4 * p.x) + (4 * b.width * p.y)
}

func (b board) ordPos(o int) position {
	xy := o / 4
	return position{
		x:      xy % b.width,
		y:      xy / b.width,
		facing: direction(o % 4),
	}
}

func repeated[E any](value E, times int) []E {
	slice := make([]E, times)
	for i := 0; i < times; i++ {
		slice[i] = value
	}
	return slice
}

func (b board) costs(cheapest int) (int, map[coord]int) {
	visited := repeated(math.MaxInt, b.width*b.height*4)
	candidates := make([]position, 0, b.width*b.height*4)
	candidates = append(candidates, b.start.facing(East))
	visited[b.posOrd(candidates[0])] = 0
	for len(candidates) > 0 {
		var pos position
		pos, candidates = candidates[0], candidates[1:]
		cost := visited[b.posOrd(pos)]
		maybeVisit2(&b, visited, &candidates, &cheapest, pos.coord().next(pos.facing), cost+1, pos.facing)
		maybeVisit2(&b, visited, &candidates, &cheapest, pos.coord(), cost+1000, pos.facing.turnCounterClockwise())
		maybeVisit2(&b, visited, &candidates, &cheapest, pos.coord(), cost+1000, pos.facing.turnClockwise())

		// sort candidates
		slices.SortStableFunc(candidates, func(p, q position) int {
			return visited[b.posOrd(q)] - visited[b.posOrd(p)]
		})
	}

	costs := make(map[coord]int)
	for idx, cost := range visited {
		if cost == math.MaxInt {
			continue
		}
		pos := b.ordPos(idx)
		if costs[pos.coord()] == 0 {
			costs[pos.coord()] = cost
		} else {
			costs[pos.coord()] = min(costs[pos.coord()], cost)
		}
	}
	return cheapest, costs
}

func (b board) reverse() board {
	b2 := b
	b2.start = b.finish
	b2.finish = b.start
	return b2
}

func part2(b board, cost int) {
	cheapest1, costs1 := b.costs(cost)
	cheapest2, costs2 := b.reverse().costs(cost)
	if cheapest1 != cheapest2 {
		panic("wtf")
	}
	path := make(map[coord]bool)
	count := 0
	for c, cost := range costs1 {
		sum := cost + costs2[c]
		if sum == cheapest1 || sum == cheapest1+1000 {
			count += 1
			path[c] = true
		}
	}
	fmt.Println("Part 2", count) // off by 2? see picture
	fmt.Println(b.stringWithPath(path))
}

func main() {
	board := readInput()
	fmt.Printf("Input:\n%s\n", board)
	cost := part1(board)
	part2(board, cost)
}
