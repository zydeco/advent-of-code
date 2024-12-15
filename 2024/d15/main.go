package main

import (
	"bufio"
	"fmt"
	"os"
)

var verbose bool = os.Getenv("VERBOSE") == "1"

type coord struct {
	x, y int
}

type item int

const (
	Empty item = iota
	Robot
	Wall
	Box
)

type board struct {
	width, height int
	tiles         map[coord]item
	robot         coord
}

type direction int

const (
	Up direction = iota
	Right
	Down
	Left
)

func readInput() (board, []direction) {
	scanner := bufio.NewScanner(os.Stdin)
	board := board{tiles: make(map[coord]item)}
	moves := make([]direction, 0, 100)

	// read rules
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
				board.tiles[xy] = Wall
			case 'O':
				board.tiles[xy] = Box
			case '@':
				board.tiles[xy] = Robot
				board.robot = xy
			}
		}
		board.height += 1
		y += 1
	}

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			panic("end of input")
		}
		for _, m := range line {
			switch m {
			case 'v':
				moves = append(moves, Down)
			case '^':
				moves = append(moves, Up)
			case '<':
				moves = append(moves, Left)
			case '>':
				moves = append(moves, Right)
			}
		}
	}
	return board, moves
}

func (d direction) reverse() direction {
	switch d {
	case Up:
		return Down
	case Down:
		return Up
	case Left:
		return Right
	case Right:
		return Left
	}
	panic("invalid direction")
}

func (d direction) String() string {
	switch d {
	case Up:
		return "^"
	case Right:
		return ">"
	case Down:
		return "v"
	case Left:
		return "<"
	}
	return "?"
}

func (c *coord) next(dir direction) coord {
	switch dir {
	case Up:
		return coord{x: c.x, y: c.y - 1}
	case Right:
		return coord{x: c.x + 1, y: c.y}
	case Down:
		return coord{x: c.x, y: c.y + 1}
	case Left:
		return coord{x: c.x - 1, y: c.y}
	default:
		panic("Invalid direction")
	}
}

func (b *board) get(c coord) item {
	if b.isInBoard(c) {
		return b.tiles[c]
	}
	return Wall
}

func (b *board) isInBoard(c coord) bool {
	return c.x >= 0 && c.y >= 0 && c.x < b.width && c.y < b.height
}

func (b *board) set(c coord, i item) {
	if i == Empty {
		b.clear(c)
	} else {
		b.tiles[c] = i
	}
}

func (b *board) clear(c coord) {
	delete(b.tiles, c)
}

func (b board) String() string {
	s := ""
	for y := 0; y < b.height; y++ {
		for x := 0; x < b.width; x++ {
			xy := coord{x: x, y: y}
			switch b.get(xy) {
			case Empty:
				s += "."
			case Wall:
				s += "#"
			case Box:
				s += "O"
			case Robot:
				s += "@"
			}
		}
		s += "\n"
	}
	return s
}

func (b board) canMove(d direction) (bool, coord) {
	target := b.robot.next(d)
	for {
		switch b.get(target) {
		case Empty:
			return true, target
		case Box:
			target = target.next(d)
		case Wall:
			return false, target
		case Robot:
			panic("invalid geometry")
		}
	}
}

func (c coord) gps() int {
	return 100*c.y + c.x
}

func (b board) gps() int {
	sum := 0
	for c, t := range b.tiles {
		if t == Box {
			sum += c.gps()
		}
	}
	return sum
}

func (b *board) move(d direction) bool {
	canMove, target := b.canMove(d)
	if !canMove {
		return false
	}

	rd := d.reverse()
	b.clear(b.robot)
	b.robot = b.robot.next(d)
	for target != b.robot {
		next := target.next(rd)
		b.set(target, b.get(next))
		b.clear(next)
		target = next
	}
	b.set(b.robot, Robot)

	return true
}

func vprintf(format string, a ...any) {
	if verbose {
		fmt.Printf(format, a...)
	}
}

func part1(b board, moves []direction) {
	for _, m := range moves {
		b.move(m)
		vprintf("Move %s:\n%s\n", m, b)
	}

	if !verbose {
		fmt.Printf("End state:\n%s\n", b)
	}
	gps := b.gps()
	fmt.Println("Part 1:", gps)
}

func main() {
	board, moves := readInput()
	fmt.Printf("Input:\n%s\n", board)
	part1(board, moves)
}
