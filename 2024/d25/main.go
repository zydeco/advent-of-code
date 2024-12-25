package main

import (
	"bufio"
	"fmt"
	"os"
)

// encode as 25 bits
type key uint32
type lock uint32

func (l lock) fits(k key) bool {
	return uint32(l)&uint32(k) == 0
}

func parseLine(line string) uint32 {
	value := uint32(0)
	for i, c := range line {
		if c == '#' {
			value |= 1 << i
		}
	}
	return value
}

func parseLockOrKey(lines []string) uint32 {
	if len(lines) != 5 {
		panic("Invalid lock or key size")
	}
	value := uint32(0)
	for _, line := range lines {
		value = value<<5 | parseLine(line)
	}
	return value
}

func readInput() ([]key, []lock) {
	keys := make([]key, 0, 32)
	locks := make([]lock, 0, 32)

	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		// read first line
		firstLine := scanner.Text()
		// read actual stuff
		dataLines := make([]string, 0, 5)
		for i := 0; i < 5; i++ {
			scanner.Scan()
			dataLines = append(dataLines, scanner.Text())
		}

		// read last line and parse
		scanner.Scan()
		lastLine := scanner.Text()
		if firstLine == "#####" && lastLine == "....." {
			locks = append(locks, lock(parseLockOrKey(dataLines)))
		} else if firstLine == "....." && lastLine == "#####" {
			keys = append(keys, key(parseLockOrKey(dataLines)))
		} else {
			panic("Invalid first or last line")
		}

		// read empty line in between
		if scanner.Scan() && scanner.Text() != "" {
			panic("Unexpected non-empty line")
		}
	}

	return keys, locks
}

func main() {
	keys, locks := readInput()
	part1(keys, locks)
}

func part1(keys []key, locks []lock) {
	nonOverlapping := 0
	for i := 0; i < len(keys); i++ {
		for j := 0; j < len(locks); j++ {
			if locks[j].fits(keys[i]) {
				nonOverlapping += 1
			}
		}
	}
	fmt.Println("Part 1:", nonOverlapping)
}
