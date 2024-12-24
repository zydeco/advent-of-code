package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
	"strings"
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

func getResult(values map[string]bool) int {
	result := 0
	for i := 0; i < 63; i++ {
		name := fmt.Sprintf("z%02d", i)
		value, hasValue := values[name]
		if !hasValue {
			break
		}
		result |= (asBit(value) << i)
	}
	return result
}

func part1(initialValues map[string]bool, gates map[string]gate) {
	values := getAllValues(initialValues, gates)
	result := getResult(values)
	fmt.Printf("Part 1: %d\n", result)
}

func add(x, y int, gates map[string]gate) int {
	values := make(map[string]bool)
	for bit := 0; bit < 45; bit++ {
		value := 1 << bit
		values[fmt.Sprintf("x%02d", bit)] = x&value != 0
		values[fmt.Sprintf("y%02d", bit)] = y&value != 0
	}
	return getResult(getAllValues(values, gates))
}

func canAdd(x, y int, gates map[string]gate) bool {
	if x >= 1<<45 || y >= 1<<45 {
		panic("invalid operands")
	}
	return add(x, y, gates) == x+y
}

func swap(a, b string, gates *map[string]gate) {
	tmp := (*gates)[a]
	(*gates)[a] = (*gates)[b]
	(*gates)[b] = tmp
}

func dependents(value string, gates map[string]gate) map[string]bool {
	dependents := make(map[string]bool)
	next := make([]string, 0)
	next = append(next, value)
	for len(next) > 0 {
		gate, hasGate := gates[next[0]]
		next = next[1:]
		if !hasGate {
			continue
		}
		if !dependents[gate.input[0]] {
			dependents[gate.input[0]] = true
			next = append(next, gate.input[0])
		}
		if !dependents[gate.input[1]] {
			dependents[gate.input[1]] = true
			next = append(next, gate.input[1])
		}
	}
	return dependents
}

func symDiff(a, b map[string]bool) map[string]bool {
	result := make(map[string]bool)
	for key := range a {
		if !b[key] {
			result[key] = true
		}
	}
	for key := range b {
		if !a[key] {
			result[key] = true
		}
	}
	return result
}

func mapKeysSorted(m map[string]bool, excludeInput bool) []string {
	keys := make([]string, 0)
	for key := range m {
		if excludeInput && (key[0] == 'x' || key[0] == 'y') {
			continue
		}
		keys = append(keys, key)
	}
	slices.Sort(keys)
	return keys
}

func canAddBit(bit int, gates map[string]gate) bool {
	if bit > 0 && !canAddBit(bit-1, gates) {
		return false
	}
	if bit < 0 {
		return true
	}
	p0 := 1 << bit
	p1 := p0 - 1
	p2 := p0 | p0>>1
	p3 := 0x55555555 & p1
	p4 := 0xAAAAAAAA & p1
	return canAdd(p0, p0, gates) &&
		canAdd(0, p0, gates) &&
		canAdd(p0, 0, gates) &&
		canAdd(p0, p1, gates) &&
		canAdd(p1, p0, gates) &&
		canAdd(p2, p0, gates) &&
		canAdd(p2, p2, gates) &&
		canAdd(p0, p2, gates) &&
		canAdd(p0, p3, gates) &&
		canAdd(p3, p4, gates)
}

func (o operation) mermaidShape() string {
	switch o {
	case And:
		return "delay"
	case Or:
		return "odd"
	case Xor:
		return "lin-rect"
	}
	panic("invalid operation")
}

func part2(initialValues map[string]bool, gates map[string]gate) {
	// print mermaid and solve graphically
	fmt.Println("flowchart LR")
	for _, key := range mapKeysSorted(initialValues, false) {
		fmt.Printf("    %s@{ shape: circle, label: \"%s\"}\n", key, key)
	}
	for output, gate := range gates {
		fmt.Printf("    %s@{ shape: %s, label: \"%s\"}\n", output, gate.op.mermaidShape(), output)
		fmt.Printf("    %s -- %s --> %s\n", gate.input[0], gate.input[0], output)
		fmt.Printf("    %s -- %s --> %s\n", gate.input[1], gate.input[1], output)
	}
}

func verify(gates map[string]gate) {
	//swap("gdd", "z05", &gates)
	//swap("cwt", "z09", &gates)
	//swap("jmv", "css", &gates)
	//swap("z37", "pqt", &gates)
	for bit := 0; bit < 45; bit++ {
		if !canAddBit(bit, gates) && !canAddBit(bit-1, gates) {
			fmt.Printf("Bad add at z%d\n", bit)
			deps := dependents(fmt.Sprintf("z%02d", bit+1), gates)
			deps[fmt.Sprintf("z%02d", bit)] = true
			sd := mapKeysSorted(deps, true)
			fmt.Printf("Candidates: %s\n", strings.Join(sd, ","))
			return
		}
	}
}

func main() {
	initialValues, gates := readInput()
	part1(initialValues, gates)
	part2(initialValues, gates)
	verify(gates)
}
