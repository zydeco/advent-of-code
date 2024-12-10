package main

import (
	"bufio"
	"fmt"
	"os"

	combinations "github.com/mxschmitt/golang-combinations"
)

type coord struct {
	x, y int
}

type board struct {
	width, height int
	antennas      map[coord]rune
}

func readInput() board {
	scanner := bufio.NewScanner(os.Stdin)
	board := board{antennas: make(map[coord]rune)}

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
			if z != '.' {
				board.antennas[xy] = z
			}
		}
		board.height += 1
		y += 1
	}

	return board
}

func (b board) String() string {
	s := ""
	for y := 0; y < b.height; y++ {
		line := ""
		for x := 0; x < b.width; x++ {
			xy := coord{x: x, y: y}
			z := b.antennas[xy]
			if z == 0 {
				line += "."
			} else {
				line += string(z)
			}
		}
		if y < b.height-1 {
			line += "\n"
		}
		s += line
	}
	return s
}

func (b board) hasCoord(c coord) bool {
	return c.x >= 0 && c.y >= 0 && c.x < b.width && c.y < b.height
}

func (b board) countAntinodes(antinodeCoords func(coord, coord) []coord) int {
	// group antennas by frequency
	antennas := make(map[rune][]coord)
	for pos, freq := range b.antennas {
		if antennas[freq] == nil {
			antennas[freq] = make([]coord, 0)
		}
		antennas[freq] = append(antennas[freq], pos)
	}

	// iterate through combinations
	antinodes := make(map[coord]bool)
	for _, coords := range antennas {
		pairs := combinations.Combinations(coords, 2)
		for _, pair := range pairs {
			for _, antinode := range antinodeCoords(pair[0], pair[1]) {
				if b.hasCoord(antinode) {
					antinodes[antinode] = true
				}
			}
		}
	}

	return len(antinodes)
}

func antinodeCoords2(c1, c2 coord) []coord {
	dx := c1.x - c2.x
	dy := c1.y - c2.y

	return []coord{
		{
			x: c1.x + dx,
			y: c1.y + dy,
		},
		{
			x: c2.x - dx,
			y: c2.y - dy,
		},
	}
}

func (b board) antinodeCoordsMax(c1, c2 coord) []coord {
	coords := make([]coord, 0, 4)
	dx := c1.x - c2.x
	dy := c1.y - c2.y

	m := 0
	for {
		c := coord{x: c1.x + (m * dx), y: c1.y + (m * dy)}
		d := coord{x: c2.x - (m * dx), y: c2.y - (m * dy)}

		coords = append(coords, c)
		coords = append(coords, d)

		if !(b.hasCoord(c) || b.hasCoord(d)) {
			break
		}
		m += 1
	}
	return coords
}

func part1(b board) {
	antinodes := b.countAntinodes(antinodeCoords2)
	fmt.Println("Antinodes Part 1:", antinodes)
}

func part2(b board) {
	antinodes := b.countAntinodes(b.antinodeCoordsMax)
	fmt.Println("Antinodes Part 2:", antinodes)
	if b.width > 10 && antinodes >= 1334 {
		fmt.Println("Too high")
	}
}

func main() {
	board := readInput()
	fmt.Println("Input:", board)
	part1(board)
	part2(board) // 1334 is too high
}
