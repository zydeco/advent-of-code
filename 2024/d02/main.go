package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type report struct {
	levels []int64
}

func (r *report) isSafe() bool {
	multiplier := 1 // assume ascending
	if r.levels[1] < r.levels[0] {
		// actually descending
		multiplier = -1
	}
	for i := 0; i < len(r.levels)-1; i++ {
		diff := (r.levels[i+1] - r.levels[i]) * int64(multiplier)
		if diff < 1 || diff > 3 {
			return false
		}
	}
	return true
}

func (r *report) without(skipIndex int) *report {
	levels := make([]int64, 0, len(r.levels)-1)
	for i := 0; i < len(r.levels); i++ {
		if i != skipIndex {
			levels = append(levels, r.levels[i])
		}
	}
	return &report{levels: levels}
}

func (r *report) isSafeWithDampening() bool {
	if r.isSafe() {
		return true
	}
	for skip := 0; skip < len(r.levels); skip++ {
		if r.without(skip).isSafe() {
			return true
		}
	}
	return false
}

func readInput() []report {
	scanner := bufio.NewScanner(os.Stdin)
	reports := make([]report, 0, 100)
	for scanner.Scan() {
		line := scanner.Text()
		words := strings.Split(line, " ")
		levels := make([]int64, 0, len(words))
		for _, word := range words {
			num, err := strconv.ParseInt(word, 10, 0)
			if err != nil {
				panic("Invalid input")
			}
			levels = append(levels, num)
		}
		reports = append(reports, report{levels: levels})
	}
	return reports
}

func main() {
	input := readInput()
	safe := 0
	safeWithDampening := 0
	for i := 0; i < len(input); i++ {
		if input[i].isSafe() {
			safe += 1
		}
		if input[i].isSafeWithDampening() {
			safeWithDampening += 1
		}
	}
	fmt.Println("Safe reports: ", safe)
	fmt.Println("Safe wit dampening: ", safeWithDampening)
}
