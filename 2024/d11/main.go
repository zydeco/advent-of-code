package main

import (
	"bufio"
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

func readInput() []int64 {
	scanner := bufio.NewScanner(os.Stdin)
	stones := make([]int64, 0, 10)
	scanner.Scan()
	line := scanner.Text()
	if line == "" {
		panic("empty line")
	}
	words := strings.Split(line, " ")
	for _, word := range words {
		value, err := strconv.ParseInt(word, 10, 0)
		if err != nil {
			panic(err)
		}
		stones = append(stones, value)
	}
	return stones
}

func numberOfDigits(i int64) int {
	digits := 1
	max := int64(10)
	for i >= max {
		digits += 1
		max *= 10
		if max < 0 {
			panic("overflow")
		}
	}
	return digits
}

func blink(stones []int64) []int64 {
	result := make([]int64, 0, len(stones))
	for _, stone := range stones {
		if stone == 0 {
			result = append(result, 1)
		} else {
			digits := numberOfDigits(stone)
			if digits%2 == 0 {
				divisor := int64(math.Pow10(digits / 2))
				right := stone % divisor
				left := stone / divisor
				result = append(result, left, right)
			} else {
				result = append(result, stone*2024)
			}
		}
	}
	return result
}

func blinkTimes(stones []int64, times int) int {
	if times <= 25 {
		for i := 0; i < times; i++ {
			stones = blink(stones)
		}
		return len(stones)
	}

	counts := make(map[int64]int)
	for _, stone := range stones {
		counts[stone] += 1
	}
	for i := 0; i < times; i += 1 {
		newCounts := make(map[int64]int)
		for stone, count := range counts {
			if stone == 0 {
				newCounts[1] += count
			} else {
				digits := numberOfDigits(stone)
				if digits%2 == 0 {
					divisor := int64(math.Pow10(digits / 2))
					right := stone % divisor
					left := stone / divisor
					newCounts[left] += count
					newCounts[right] += count
				} else {
					newCounts[stone*2024] += count
				}
			}
		}
		counts = newCounts
	}

	total := 0
	for _, count := range counts {
		total += count
	}
	return total
}

func main() {
	stones := readInput()
	part1 := blinkTimes(stones, 25)
	fmt.Println("Part 1:", part1)
	part2 := blinkTimes(stones, 75)
	fmt.Println("Part 2:", part2)
}
