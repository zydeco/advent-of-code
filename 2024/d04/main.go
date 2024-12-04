package main

import (
	"bufio"
	"fmt"
	"os"
)

type board struct {
	// tile[y][x]
	tile [][]rune
}

type pos struct {
	x, y int
}

var (
	Up            pos = pos{x: 0, y: -1}
	UpLeft        pos = pos{x: -1, y: -1}
	Left          pos = pos{x: -1, y: 0}
	DownLeft      pos = pos{x: -1, y: 1}
	Down          pos = pos{x: 0, y: 1}
	DownRight     pos = pos{x: 1, y: 1}
	Right         pos = pos{x: 1, y: 0}
	UpRight       pos = pos{x: 1, y: -1}
	AllDirections     = []pos{Up, UpLeft, Left, DownLeft, Down, DownRight, Right, UpRight}
)

func (a *pos) add(b pos) pos {
	return pos{x: a.x + b.x, y: a.y + b.y}
}

func (b *board) getXY(x, y int) rune {
	if x < 0 || y < 0 || y >= len(b.tile) || x >= len(b.tile[0]) {
		return 0
	}
	return b.tile[y][x]
}

func (b *board) get(p pos) rune {
	return b.getXY(p.x, p.y)
}

func (b *board) width() int {
	return len(b.tile[0])
}

func (b *board) height() int {
	return len(b.tile)
}

func (b *board) hasWord(word string, start, direction pos) bool {
	pos := start
	for _, letter := range word {
		if b.get(pos) != letter {
			return false
		}
		pos = pos.add(direction)
	}
	return true
}

type ray struct {
	position, direction pos
}

func (b *board) findWord(word string) []ray {
	results := make([]ray, 0, 1)
	for y := 0; y < b.height(); y++ {
		for x := 0; x < b.width(); x++ {
			xy := pos{x: x, y: y}
			for _, dir := range AllDirections {
				if b.hasWord(word, xy, dir) {
					results = append(results, ray{position: xy, direction: dir})
				}
			}
		}
	}
	return results
}

func (b *board) countXmas() int {
	result := 0
	for y := 0; y < b.height(); y++ {
		for x := 0; x < b.width(); x++ {
			xy := pos{x: x, y: y}
			if b.get(xy) == 'A' {
				nw, ne, sw, se := b.get(xy.add(UpLeft)), b.get(xy.add(UpRight)), b.get(xy.add(DownLeft)), b.get(xy.add(DownRight))
				if ((nw == 'M' && se == 'S') || (nw == 'S' && se == 'M')) && ((ne == 'M' && sw == 'S') || (ne == 'S' && sw == 'M')) {
					result += 1
				}
			}
		}
	}
	return result
}

func readInput() board {
	scanner := bufio.NewScanner(os.Stdin)
	tile := make([][]rune, 0, 140)
	for scanner.Scan() {
		line := scanner.Text()
		tile = append(tile, []rune(line))
	}
	return board{tile: tile}
}

func main() {
	board := readInput()
	xmases := board.findWord("XMAS")
	fmt.Println("Found", len(xmases), "times")
	fmt.Println("Part 2:", board.countXmas(), "times")

}
