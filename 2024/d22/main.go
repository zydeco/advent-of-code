package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func readInput() []int {
	numbers := make([]int, 0, 4)
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		number, err := strconv.Atoi(scanner.Text())
		if err != nil {
			panic(err)
		}
		numbers = append(numbers, number)
	}
	return numbers
}

func mix(a, b int) int {
	return a ^ b
}

func prune(x int) int {
	// x % 0x1000000 === x & 0xffffff
	return x & 0xffffff
}

func next(number int) int {
	// x * 64 === x << 6
	// x / 32 === x >> 5
	// x * 2048 === x << 11
	number = prune(mix(number, number<<6))
	number = prune(mix(number, number>>5))
	number = prune(mix(number, number<<11))
	return number
}

func nth(number, n int) int {
	for i := 0; i < n; i++ {
		number = next(number)
	}
	return number
}

func part1(input []int) {
	sum := 0
	for _, number := range input {
		sum += nth(number&0xffffff, 2000)
	}
	fmt.Println("Part 1", sum)
}

type sequence [4]int8

func (s sequence) put(next int8) sequence {
	return sequence{s[1], s[2], s[3], next}
}

func pricesBySequence(number, n int) map[sequence]int {
	seq := sequence{}
	prices := make(map[sequence]int)
	lastPrice := number % 10
	for i := 0; i < n; i++ {
		number = next(number)
		nextPrice := number % 10
		seq = seq.put(int8(nextPrice - lastPrice))
		_, hasPrice := prices[seq]
		if !hasPrice && i >= 3 {
			// first occurrence of this sequence
			// must be at 4th iteration or later
			prices[seq] = nextPrice
		}
		lastPrice = nextPrice
	}
	return prices
}

func part2(input []int) {
	bananasPerSequence := make(map[sequence]int)
	for _, number := range input {
		for sequence, bananas := range pricesBySequence(number, 2000) {
			bananasPerSequence[sequence] += bananas
		}
	}
	maxBananas := 0
	for _, bananas := range bananasPerSequence {
		maxBananas = max(maxBananas, bananas)
	}
	fmt.Println("Part 2", maxBananas)
}

func main() {
	input := readInput()
	part1(input)
	part2(input)
}
