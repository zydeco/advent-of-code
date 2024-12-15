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
	BoxLeft
	BoxRight
)

type board struct {
	width, height int
	tiles         map[coord]item
	robot         coord
	wideBoxes     bool
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

func (c coord) next(dir direction) coord {
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
	s := " "
	for x := 0; x < b.width; x++ {
		s += fmt.Sprintf("%d", x%10)
	}
	s += "\n"
	for y := 0; y < b.height; y++ {
		s += fmt.Sprintf("%d", y%10)
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
			case BoxLeft:
				s += "["
			case BoxRight:
				s += "]"
			}
		}
		s += "\n"
	}
	return s
}

func (d direction) isVertical() bool {
	return d == Up || d == Down
}

func (b board) canMove(d direction) (bool, coord) {
	target := b.robot.next(d)
	for {
		switch b.get(target) {
		case Empty:
			return true, target
		case Box:
			target = target.next(d)
		case BoxLeft, BoxRight:
			if d.isVertical() {
				return b.canMoveBigBoxVertically(target, d), target
			}
			target = target.next(d)
		case Wall:
			return false, target
		case Robot:
			panic("invalid geometry")
		}
	}
}

func (b *board) canMoveBigBoxVertically(c coord, d direction) bool {
	return b.moveBigBoxVertically(c, d, true)
}

func (b *board) moveBigBox(to, from coord) {
	b.clear(from)
	b.clear(from.next(Right))
	b.set(to, BoxLeft)
	b.set(to.next(Right), BoxRight)
}

func (b *board) moveBigBoxVertically(c coord, d direction, dryRun bool) bool {
	// calculate next positions
	if b.get(c) == BoxRight {
		return b.moveBigBoxVertically(c.next(Left), d, dryRun)
	}
	if b.get(c) != BoxLeft {
		panic("wtf")
	}

	// calculate next positions
	c1 := c.next(d)
	c2 := c.next(Right).next(d)
	t1 := b.get(c1)
	t2 := b.get(c2)

	// if any is wall -> no
	// if any is box -> recurse
	if t1 == Empty && t2 == Empty {
		if !dryRun {
			b.moveBigBox(c1, c)
		}
		return true
	} else if t1 == Wall || t2 == Wall {
		return false
	} else if t1 == BoxLeft && t2 == BoxRight {
		if b.moveBigBoxVertically(c1, d, dryRun) {
			if !dryRun {
				b.moveBigBox(c1, c)
			}
			return true
		}
		return false
	} else if t1 == BoxRight || t2 == BoxLeft {
		if t1 == BoxRight && !b.moveBigBoxVertically(c1.next(Left), d, dryRun) {
			return false
		}
		if t2 == BoxLeft && !b.moveBigBoxVertically(c2, d, dryRun) {
			return false
		}
		if !dryRun {
			b.moveBigBox(c1, c)
		}
		return true
	}
	panic("unforeseen box situation")
}

func (c coord) gps() int {
	return 100*c.y + c.x
}

func (b board) gps() int {
	sum := 0
	for c, t := range b.tiles {
		if t == Box || t == BoxLeft {
			sum += c.gps()
		}
	}
	return sum
}

func (i item) isBigBox() bool {
	return i == BoxLeft || i == BoxRight
}

func (b *board) move(d direction) bool {
	canMove, target := b.canMove(d)
	if !canMove {
		return false
	}

	b.clear(b.robot)
	b.robot = b.robot.next(d)

	if b.wideBoxes && d.isVertical() && b.get(b.robot).isBigBox() {
		b.moveBigBoxVertically(b.robot, d, false)
	} else {
		rd := d.reverse()
		for target != b.robot {
			next := target.next(rd)
			b.set(target, b.get(next))
			b.clear(next)
			target = next
		}
	}

	b.set(b.robot, Robot)

	return true
}

func vprintf(format string, a ...any) {
	if verbose {
		fmt.Printf(format, a...)
	}
}

func (c coord) embiggen() coord {
	return coord{x: c.x * 2, y: c.y}
}

func (b board) embiggen() board {
	b2 := board{
		width:     b.width * 2,
		height:    b.height,
		tiles:     make(map[coord]item),
		robot:     b.robot.embiggen(),
		wideBoxes: true,
	}
	for c, t := range b.tiles {
		c2 := c.embiggen()
		c3 := c2.next(Right)
		switch t {
		case Wall:
			b2.tiles[c2] = Wall
			b2.tiles[c3] = Wall
		case Box:
			b2.tiles[c2] = BoxLeft
			b2.tiles[c3] = BoxRight
		}
	}
	b2.tiles[b2.robot] = Robot
	return b2
}

func (b board) copy() board {
	b2 := b
	b2.tiles = make(map[coord]item)
	for c, t := range b.tiles {
		b2.tiles[c] = t
	}
	return b2
}

func doMoves(b board, moves []direction) {
	b = b.copy()
	fmt.Printf("Initial State:\n%s\n", b)

	for _, m := range moves {
		b.move(m)
		vprintf("Move %s:\n%s\n", m, b)
	}

	if !verbose {
		fmt.Printf("End state:\n%s\n", b)
	}
	gps := b.gps()
	fmt.Println("Result:", gps)
}

func main() {
	board, moves := readInput()
	fmt.Println("Part 1:")
	doMoves(board, moves)

	fmt.Println("Part 2:")
	board2 := board.embiggen()
	doMoves(board2, moves)
}
