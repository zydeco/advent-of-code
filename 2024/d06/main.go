package main

import (
	"bufio"
	"fmt"
	"os"
)

type coord struct {
	x, y int
}

type board struct {
	width, height int
	tiles         map[coord]rune
}

type direction int

const (
	Up direction = iota
	Right
	Down
	Left
)

type guard struct {
	position coord
	facing   direction
}

func readInput() (board, guard) {
	scanner := bufio.NewScanner(os.Stdin)
	board := board{tiles: make(map[coord]rune)}
	guard := guard{}

	// read rules
	y := 0
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			panic("empty line")
		}
		if board.width == 0 {
			board.width = len(line)
		}

		for x, z := range line {
			xy := coord{x: x, y: y}
			switch z {
			case '#':
				board.tiles[xy] = '#'
			case '^':
				guard.position = xy
				guard.facing = Up
			}
		}
		board.height += 1
		y += 1
	}

	return board, guard
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

func (d direction) turn() direction {
	switch d {
	case Up:
		return Right
	case Right:
		return Down
	case Down:
		return Left
	case Left:
		return Up
	default:
		panic("Invalid direction")
	}
}

func (b *board) isFree(c coord) bool {
	return b.tiles[c] == 0
}

func (b *board) isInBoard(c coord) bool {
	return c.x >= 0 && c.y >= 0 && c.x < b.width && c.y < b.height
}

func (b *board) block(c coord) {
	b.tiles[c] = 'O'
}

func (b *board) clear(c coord) {
	delete(b.tiles, c)
}

func (g guard) move(b *board) guard {
	facing := g.facing
	nextPos := g.position.next(facing)
	for i := 0; i < 3; i++ {
		if !b.isFree(nextPos) {
			facing = facing.turn()
			nextPos = g.position.next(facing)
		}
	}
	if !b.isFree(nextPos) {
		panic("How did I get there?")
	}
	return guard{position: nextPos, facing: facing}
}

func printState(b *board, g *guard) {
	for y := 0; y < b.height; y++ {
		for x := 0; x < b.width; x++ {
			xy := coord{x: x, y: y}
			if g.position == xy {
				fmt.Print(g.facing)
				continue
			}
			tile := b.tiles[xy]
			if tile == 0 {
				fmt.Print(".")
			} else {
				fmt.Printf("%c", tile)
			}
		}
		fmt.Println("")
	}
}

func part1(b *board, g guard) {
	visited := make(map[coord]bool)
	for b.isInBoard(g.position) {
		visited[g.position] = true
		g = g.move(b)
	}
	fmt.Println("Visited", len(visited), "positions")
}

func part2(b *board, g guard) {
	// go through path normally
	// at each step, block the front
	// see if that loops
	start := g.position
	loops := make(map[coord]bool)
	visited := make(map[coord]bool)
	for b.isInBoard(g.position) {
		visited[g.position] = true
		next := g.move(b)
		if next.position != start && b.isInBoard(next.position) && !visited[next.position] {
			// block path
			b.block(next.position)
			if b.loops(g) {
				loops[next.position] = true
			}
			b.clear(next.position)
		}
		g = next
	}
	fmt.Println("Found loops:", len(loops))
}

func (b *board) loops(start guard) bool {
	g := start
	visited := make(map[guard]bool)
	for b.isInBoard(g.position) {
		if visited[g] {
			return true
		}
		visited[g] = true
		g = g.move(b)
	}
	return false
}

func main() {
	board, guard := readInput()
	part1(&board, guard)
	part2(&board, guard)
}
