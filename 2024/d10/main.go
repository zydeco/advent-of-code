package main

import (
	"bufio"
	"fmt"
	"os"
)

type heightmap [][]int8

type coord struct {
	x, y int
}

func (h heightmap) size() (int, int) {
	return len(h[0]), len(h)
}

func (h heightmap) hasCoord(c coord) bool {
	hw, hh := h.size()
	return c.x >= 0 && c.y >= 0 && c.x < hw && c.y < hh
}

func (h heightmap) get(c coord) int8 {
	if h.hasCoord(c) {
		return h[c.y][c.x]
	}
	return -1
}

func (c coord) north() coord {
	return coord{x: c.x, y: c.y - 1}
}

func (c coord) south() coord {
	return coord{x: c.x, y: c.y + 1}
}

func (c coord) east() coord {
	return coord{x: c.x - 1, y: c.y}
}

func (c coord) west() coord {
	return coord{x: c.x + 1, y: c.y}
}

func readInput() heightmap {
	scanner := bufio.NewScanner(os.Stdin)
	heights := make([][]int8, 0, 60)

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			panic("empty line")
		}
		items := make([]int8, len(line))
		for i := 0; i < len(line); i++ {
			if line[i] == '.' {
				items[i] = -1
			} else {
				items[i] = int8(line[i] - '0')
			}
		}
		heights = append(heights, items)
	}
	return heights
}

func (h heightmap) reachable9s(from coord) []coord {
	cv := h.get(from)
	if cv == 9 {
		return []coord{from}
	}
	reached := make(map[coord]bool)
	borders := []coord{from.north(), from.east(), from.south(), from.west()}
	for _, pos := range borders {
		if h.get(pos) == cv+1 {
			for _, coord := range h.reachable9s(pos) {
				reached[coord] = true
			}
		}
	}
	reachedList := make([]coord, 0, len(reached))
	for coord, _ := range reached {
		reachedList = append(reachedList, coord)
	}
	return reachedList
}

func (h heightmap) rating(from coord) int {
	cv := h.get(from)
	if cv == 9 {
		return 1
	}
	rating := 0
	borders := []coord{from.north(), from.east(), from.south(), from.west()}
	for _, pos := range borders {
		if h.get(pos) == cv+1 {
			rating += h.rating(pos)
		}
	}
	return rating
}

func calculateScore(h heightmap, scoreFunc func(coord) int) int {
	score := 0
	sx, sy := h.size()
	for y := 0; y < sy; y++ {
		for x := 0; x < sx; x++ {
			xy := coord{x: x, y: y}
			if h.get(xy) == 0 {
				score += scoreFunc(xy)
			}
		}
	}
	return score
}

func main() {
	input := readInput()
	fmt.Println("Part 1:", calculateScore(input, func(xy coord) int {
		return len(input.reachable9s(xy))
	}))

	fmt.Println("Part 12", calculateScore(input, input.rating))
}
