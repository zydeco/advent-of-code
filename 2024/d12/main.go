package main

import (
	"bufio"
	"fmt"
	"os"
)

type coord struct {
	x, y int
}

type border struct {
	coord coord
	side  edge
}

type edge int

const (
	North edge = 0
	West  edge = 1
	South edge = 2
	East  edge = 3
)

type board [][]rune

func (b board) hasCoord(xy coord) bool {
	return xy.x >= 0 && xy.y >= 0 && xy.x < len(b[0]) && xy.y < len(b)
}

func (b board) get(xy coord) rune {
	if b.hasCoord(xy) {
		return b[xy.y][xy.x]
	}
	return 0
}

func (b board) size() (int, int) {
	width := len(b[0])
	height := len(b)
	return width, height
}

func readInput() board {
	scanner := bufio.NewScanner(os.Stdin)
	board := make([][]rune, 0, 10)

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			panic("empty line")
		}

		row := make([]rune, len(line))
		for x, z := range line {
			row[x] = z
		}
		board = append(board, row)
	}

	return board
}

func (b board) String() string {
	s := ""
	width, height := b.size()
	for y := 0; y < height; y++ {
		line := ""
		for x := 0; x < width; x++ {
			z := b.get(coord{x: x, y: y})
			if z == 0 {
				line += "."
			} else {
				line += string(z)
			}
		}
		if y < height-1 {
			line += "\n"
		}
		s += line
	}
	return s
}

func (c coord) north() coord {
	return coord{x: c.x, y: c.y - 1}
}

func (c coord) south() coord {
	return coord{x: c.x, y: c.y + 1}
}

func (c coord) east() coord {
	return coord{x: c.x + 1, y: c.y}
}

func (c coord) west() coord {
	return coord{x: c.x - 1, y: c.y}
}

// must be same order as edge enum
func (c coord) neighbours() [4]coord {
	return [...]coord{c.north(), c.west(), c.south(), c.east()}
}

func areaAndPerimeter(b board, c coord, visited map[coord]bool) (int, int) {
	target := b.get(c)
	area := 1
	perimeter := 0
	neighbours := c.neighbours()
	visited[c] = true
	for i := 0; i < 4; i++ {
		c2 := neighbours[i]
		if b.hasCoord(c2) && b.get(c2) == target {
			if !visited[c2] {
				subArea, subPerimeter := areaAndPerimeter(b, c2, visited)
				area += subArea
				perimeter += subPerimeter
			}
		} else {
			perimeter += 1
		}
	}
	return area, perimeter
}

func visitAndBorder(b board, c coord, visited map[coord]bool, borders map[border]bool) {
	target := b.get(c)
	neighbours := c.neighbours()
	visited[c] = true
	for i := 0; i < 4; i++ {
		c2 := neighbours[i]
		if b.hasCoord(c2) && b.get(c2) == target {
			if !visited[c2] {
				visitAndBorder(b, c2, visited, borders)
			}
		} else {
			borders[border{coord: c, side: edge(i)}] = true
		}
	}
}

func part1(b board) {
	visited := make(map[coord]bool)
	cols, lines := b.size()
	sum := 0
	for y := 0; y < lines; y++ {
		for x := 0; x < cols; x++ {
			xy := coord{x: x, y: y}
			if visited[xy] {
				continue
			}
			area, perimeter := areaAndPerimeter(b, xy, visited)
			//fmt.Printf("A region of %c plants with price %d * %d = %d\n", b.get(xy), area, perimeter, area*perimeter)
			sum += area * perimeter
		}
	}
	fmt.Println("Part 1:", sum)
}

func (c coord) String() string {
	return fmt.Sprintf("%d,%d", c.x, c.y)
}

func (c coord) border(e edge) border {
	return border{coord: c, side: e}
}

func (e edge) String() string {
	switch e {
	case North:
		return "north"
	case South:
		return "south"
	case East:
		return "east"
	case West:
		return "west"
	}
	panic("invalid edge")
}

func (b border) String() string {
	return fmt.Sprintf("%s|%s", b.coord, b.side)
}

func rincones(region map[coord]bool, borders map[border]bool) int {
	if len(region) == 1 {
		return 4
	}
	rincones := 0
	for c := range region {
		n, w, s, e := c.border(North), c.border(West), c.border(South), c.border(East)
		if borders[n] && borders[w] {
			rincones += 1
		}
		if borders[n] && borders[e] {
			rincones += 1
		}
		if borders[s] && borders[w] {
			rincones += 1
		}
		if borders[s] && borders[e] {
			rincones += 1
		}
	}
	return rincones
}

func esquinas(region map[coord]bool, borders map[border]bool) int {
	esquinas := 0
	for c := range region {
		n, w, s, e := c.north(), c.west(), c.south(), c.east()
		if borders[n.border(West)] && borders[w.border(North)] {
			esquinas += 1
		}
		if borders[n.border(East)] && borders[e.border(North)] {
			esquinas += 1
		}
		if borders[s.border(West)] && borders[w.border(South)] {
			esquinas += 1
		}
		if borders[s.border(East)] && borders[e.border(South)] {
			esquinas += 1
		}
	}
	return esquinas
}

func part2(b board) {
	visited := make(map[coord]bool)
	cols, lines := b.size()
	sum := 0
	for y := 0; y < lines; y++ {
		for x := 0; x < cols; x++ {
			xy := coord{x: x, y: y}
			if visited[xy] {
				continue
			}
			borders := make(map[border]bool)
			region := make(map[coord]bool)
			visitAndBorder(b, xy, region, borders)
			for c := range region {
				visited[c] = true
			}
			sides := rincones(region, borders) + esquinas(region, borders)
			area := len(region)
			//fmt.Printf("A region of %c plants at %s with price %d * %d = %d\n", b.get(xy), xy, area, sides, area*sides)
			sum += area * sides
		}
	}
	fmt.Println("Part 2:", sum)

}

func main() {
	board := readInput()
	fmt.Printf("Input:\n%s\n", board)
	part1(board)
	part2(board)
}
