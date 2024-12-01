package main

import (
	"bufio"
	"container/list"
	"fmt"
	"os"
	"slices"
)

type input struct {
	left  []int
	right []int
}

func (i *input) sort() {
	slices.SortFunc(i.left, func(a, b int) int {
		return a - b
	})
	slices.SortFunc(i.right, func(a, b int) int {
		return a - b
	})
}

func absDiff(x, y int) int {
	if x < y {
		return y - x
	}
	return x - y
}

func sum(slc []int) int {
	x := 0
	for i := 0; i < len(slc); i++ {
		x += slc[i]
	}
	return x
}

func (x *input) len() int {
	return len(x.left)
}

func (x *input) differences() *[]int {
	result := make([]int, x.len())
	for i := 0; i < len(result); i++ {
		result[i] = absDiff(x.left[i], x.right[i])
	}
	return &result
}

func (x *input) similarityScore() int {
	occurrences := make(map[int]int)
	score := 0
	for i := 0; i < x.len(); i++ {
		occurrences[x.right[i]] += 1
	}
	for i := 0; i < x.len(); i++ {
		value := x.left[i]
		score += value * occurrences[value]
	}
	return score
}

func readInput() *input {
	scanner := bufio.NewScanner(os.Stdin)
	leftList := list.New()
	rightList := list.New()

	for scanner.Scan() {
		line := scanner.Text()
		var left, right int
		fmt.Sscanf(line, "%d %d", &left, &right)
		leftList.PushBack(left)
		rightList.PushBack(right)
	}
	if err := scanner.Err(); err != nil {
		fmt.Fprintln(os.Stderr, "reading standard input:", err)
	}

	result := input{}
	result.left = make([]int, leftList.Len())
	result.right = make([]int, rightList.Len())
	l := leftList.Front()
	r := rightList.Front()
	for i := 0; i < leftList.Len(); i++ {
		result.left[i] = l.Value.(int)
		result.right[i] = r.Value.(int)
		l = l.Next()
		r = r.Next()
	}
	return &result
}

func main() {
	input := readInput()
	input.sort()
	diffs := input.differences()
	result := sum(*diffs)
	fmt.Println("Part 1:", result)
	result2 := input.similarityScore()
	fmt.Println("Part 2:", result2)
}
