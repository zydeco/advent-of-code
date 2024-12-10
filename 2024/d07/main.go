package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type equation struct {
	result uint64
	values []uint64
}

type operator int

const (
	Add operator = iota
	Multiply
	Concatenate
)

type solution []operator

func readInput() []equation {
	scanner := bufio.NewScanner(os.Stdin)
	equations := make([]equation, 0)

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			panic("empty line")
		}

		words := strings.Split(line, " ")
		values := make([]uint64, 0, len(words)-1)
		var result uint64
		for index, word := range words {
			if index == 0 {
				num, err := strconv.ParseInt(word[:len(word)-1], 10, 0)
				if err != nil {
					panic("Invalid input " + word)
				}
				result = uint64(num)
				continue
			}
			num, err := strconv.ParseInt(word, 10, 0)
			if err != nil {
				panic("Invalid input " + word)
			}
			values = append(values, uint64(num))
		}

		equations = append(equations, equation{result: result, values: values})
	}

	return equations
}

func (x operator) String() string {
	switch x {
	case Add:
		return "+"
	case Multiply:
		return "*"
	case Concatenate:
		return "||"
	}
	return "?"
}

func next10(x uint64) uint64 {
	result := uint64(10)
	for x >= result {
		result *= 10
	}
	return result
}

func (q *equation) calculate(operators *[]operator) uint64 {
	result := q.values[0]
	for i, value := range q.values[1:] {
		previousResult := result
		switch (*operators)[i] {
		case Add:
			result += value
		case Multiply:
			result *= value
		case Concatenate:
			result = result*next10(value) + value
		}
		if result < previousResult {
			panic("overflow?!")
		}
	}
	return result
}

func canIncrementOperators(ops *[]operator, lastOp operator) bool {
	for _, op := range *ops {
		if op != lastOp {
			return true
		}
	}
	return false
}

func incrementOperators(ops *[]operator, lastOp operator) bool {
	// check if it's at max
	if !canIncrementOperators(ops, lastOp) {
		return false
	}
	// increment
	for i := range *ops {
		if (*ops)[i] == lastOp {
			// carry the one
			(*ops)[i] = Add
		} else {
			(*ops)[i] += 1
			return true
		}
	}
	panic("this should be unreachable")
}

func (q *equation) solutions(lastOp operator) []solution {
	solutions := make([]solution, 0, 1)
	operators := make([]operator, len(q.values)-1)
	for {
		if q.calculate(&operators) == q.result {
			solution := make([]operator, len(operators))
			copy(solution, operators)
			solutions = append(solutions, solution)
			break // only need one solution
		}
		if !incrementOperators(&operators, lastOp) {
			break
		}
	}
	return solutions
}

func solve(equations []equation, lastOp operator) uint64 {
	var result uint64
	for _, q := range equations {
		solutions := q.solutions(lastOp)
		if len(solutions) > 0 {
			result += q.result
		}
	}
	return result
}

func main() {
	equations := readInput()
	fmt.Println("Input:", len(equations))
	p1 := solve(equations, Multiply)
	fmt.Println("Part 1:", p1)
	p2 := solve(equations, Concatenate)
	fmt.Println("Part 2:", p2)
}
