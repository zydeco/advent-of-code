package main

import (
	"bufio"
	"fmt"
	"os"
)

type operation int

const (
	And operation = iota
	Or
	Xor
)

type gate struct {
	input [2]string
	op    operation
}

func (o operation) String() string {
	switch o {
	case And:
		return "AND"
	case Or:
		return "OR"
	case Xor:
		return "XOR"
	default:
		panic("Invalid operation")
	}
}

func (g gate) String() string {
	return fmt.Sprintf("%s %s %s", g.input[0], g.op, g.input[1])
}

func parseOp(s string) operation {
	if s == "AND" {
		return And
	} else if s == "OR" {
		return Or
	} else if s == "XOR" {
		return Xor
	}
	panic("Invalid operation: " + s)
}

func readInput() (map[string]bool, map[string]gate) {
	scanner := bufio.NewScanner(os.Stdin)

	// read initial values
	initialValues := make(map[string]bool)
	for scanner.Scan() {
		if scanner.Text() == "" {
			// empty line
			break
		}

		var name string
		var value int
		_, err := fmt.Sscanf(scanner.Text(), "%3s: %d", &name, &value)
		if err != nil {
			panic("Invalid initial value definition: " + scanner.Text())
		}
		initialValues[name] = (value == 1)
	}

	// read gates
	gates := make(map[string]gate)
	for scanner.Scan() {
		if scanner.Text() == "" {
			panic("Unexpected empty line in gates")
		}
		var input1, input2, opName, output string
		_, err := fmt.Sscanf(scanner.Text(), "%s %s %s -> %s", &input1, &opName, &input2, &output)
		if err != nil {
			panic("Invalid gate definition: " + scanner.Text())
		}
		gates[output] = gate{input: [2]string{input1, input2}, op: parseOp(opName)}
	}

	return initialValues, gates
}

func (o operation) apply(a, b bool) bool {
	switch o {
	case And:
		return a && b
	case Or:
		return a || b
	case Xor:
		return a != b
	}
	panic("Undefined operation")
}

func getValue(name string, values *map[string]bool, gates *map[string]gate) bool {
	value, hasValue := (*values)[name]
	if hasValue {
		return value
	}
	// calculate value
	gate := (*gates)[name]
	value = gate.op.apply(getValue(gate.input[0], values, gates), getValue(gate.input[1], values, gates))
	(*values)[name] = value
	return value
}

func getAllValues(initialValues map[string]bool, gates map[string]gate) map[string]bool {
	values := make(map[string]bool)
	for key, value := range initialValues {
		values[key] = value
	}
	for key := range gates {
		values[key] = getValue(key, &values, &gates)
	}
	return values
}

func asBit(value bool) int {
	if value {
		return 1
	}
	return 0
}

func part1(initialValues map[string]bool, gates map[string]gate) {
	values := getAllValues(initialValues, gates)
	result := 0
	for i := 0; i < 63; i++ {
		name := fmt.Sprintf("z%02d", i)
		value, hasValue := values[name]
		if !hasValue {
			break
		}
		result |= (asBit(value) << i)
	}
	fmt.Printf("Part 1: %d\n", result)
}

func main() {
	initialValues, gates := readInput()
	fmt.Println("Intial values:", initialValues)
	fmt.Println("Gates:", gates)
	part1(initialValues, gates)
}
