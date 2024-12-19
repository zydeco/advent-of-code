package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func readInput() (map[string]bool, []string) {
	scanner := bufio.NewScanner(os.Stdin)

	// read patterns
	scanner.Scan()
	patterns := make(map[string]bool)
	for _, pattern := range strings.Split(scanner.Text(), ", ") {
		patterns[pattern] = true
	}
	scanner.Scan() // empty line
	if scanner.Text() != "" {
		panic("Expected second line to be empty")
	}

	designs := make([]string, 0, 400)
	for scanner.Scan() {
		designs = append(designs, scanner.Text())
	}

	return patterns, designs
}

func longestPattern(patterns map[string]bool) int {
	longest := 0
	for pattern := range patterns {
		longest = max(longest, len(pattern))
	}
	return longest
}

func canMakeDesign(design string, patterns map[string]bool, longestPattern int) bool {
	if len(design) == 0 {
		return true
	}
	for i := min(longestPattern, len(design)); i > 0; i-- {
		if patterns[design[0:i]] && canMakeDesign(design[i:], patterns, longestPattern) {
			return true
		}
	}
	return false
}

func designCombinations(design string, patterns map[string]bool, longestPattern int, cache *map[string]int) int {
	if len(design) == 0 {
		return 1
	}
	combinations, cached := (*cache)[design]
	if !cached {
		for i := min(longestPattern, len(design)); i > 0; i-- {
			if patterns[design[0:i]] {
				combinations += designCombinations(design[i:], patterns, longestPattern, cache)
			}
		}
		(*cache)[design] = combinations
	}
	return combinations
}

func part1(patterns map[string]bool, designs []string) {
	possible := 0
	longest := longestPattern(patterns)
	for _, design := range designs {
		if canMakeDesign(design, patterns, longest) {
			possible += 1
		}
	}
	fmt.Println("Part 1:", possible)
}

func part2(patterns map[string]bool, designs []string) {
	possible := 0
	longest := longestPattern(patterns)
	cache := make(map[string]int)
	for _, design := range designs {
		possible += designCombinations(design, patterns, longest, &cache)
	}
	fmt.Println("Part 2:", possible)
}

func main() {
	patterns, designs := readInput()
	part1(patterns, designs)
	part2(patterns, designs)
}
