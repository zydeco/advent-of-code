package main

import (
	"bufio"
	"fmt"
	"os"
	"os/exec"
	"strconv"
	"strings"
)

var verbose bool = os.Getenv("VERBOSE") == "1"

type registers struct {
	a, b, c, ip int
}

type opcode uint8

const (
	Adv opcode = 0
	Bxl opcode = 1
	Bst opcode = 2
	Jnz opcode = 3
	Bxc opcode = 4
	Out opcode = 5
	Bdv opcode = 6
	Cdv opcode = 7
)

func readInput() (registers, []uint8) {
	scanner := bufio.NewScanner(os.Stdin)
	regs := make([]int, 3)
	for i := 0; i < 3; i++ {
		scanner.Scan() // register
		var registerName rune
		fmt.Sscanf(scanner.Text(), "Register %c: %d", &registerName, &regs[i])
	}
	scanner.Scan() // empty line
	scanner.Scan() // program
	words := strings.Split(scanner.Text()[len("Program: "):], ",")
	program := make([]uint8, len(words))
	for i, word := range words {
		value, _ := strconv.Atoi(word)
		program[i] = uint8(value)
	}
	return registers{a: regs[0], b: regs[1], c: regs[2]}, program
}

func comboOperand(r *registers, value uint8) int {
	switch value {
	case 4:
		return r.a
	case 5:
		return r.b
	case 6:
		return r.c
	default:
		if value >= 7 {
			panic("invalid operand")
		}
		return int(value)
	}
}

func pow2(x int) int {
	return 1 << x
}

func xdv(r *registers, numerator int, operand uint8) int {
	denominator := pow2(comboOperand(r, operand))
	if denominator == 0 {
		return 0
	}
	return numerator / denominator
}

const (
	NoOutput = uint8(8)
)

// returns output value or NoOutput
func step(r *registers, mem []uint8) uint8 {
	operand := mem[r.ip+1]
	switch opcode(mem[r.ip]) {
	case Adv:
		r.a = xdv(r, r.a, operand)
		r.ip += 2
	case Bxl:
		r.b ^= int(operand)
		r.ip += 2
	case Bst:
		r.b = comboOperand(r, operand) % 8
		r.ip += 2
	case Jnz:
		if r.a == 0 {
			r.ip += 2
		} else {
			r.ip = int(operand)
		}
	case Bxc:
		r.b ^= r.c
		r.ip += 2
	case Out:
		output := uint8(comboOperand(r, operand) % 8)
		r.ip += 2
		return output
	case Bdv:
		r.b = xdv(r, r.a, operand)
		r.ip += 2
	case Cdv:
		r.c = xdv(r, r.a, operand)
		r.ip += 2
	}

	return NoOutput
}

func part1(r registers, program []uint8) {
	output := make([]uint8, 0, 10)
	for r.ip < len(program) {
		out := step(&r, program)
		if out != NoOutput {
			output = append(output, out)
		}
	}
	fmt.Println("Part 1:", strings.ReplaceAll(fmt.Sprint(output), " ", ","))
}

func isQuine(a int, program []uint8) bool {
	op := 0 // output pointer
	r := registers{a: a}
	for r.ip < len(program) {
		out := step(&r, program)
		if out != NoOutput {
			if out == program[op] {
				op++
				if op == len(program) {
					return true
				}
			} else {
				return false
			}
		}
	}
	return false
}

func z3(script string, wantedResults ...string) (bool, []int) {
	script += "\n(check-sat)"
	results := make([]int, len(wantedResults))
	for _, name := range wantedResults {
		script += fmt.Sprintf("\n(eval %s)", name)
	}
	if verbose {
		fmt.Println(script)
	}
	cmd := exec.Command("z3", "-in")
	cmd.Stdin = strings.NewReader(script)
	out, err := cmd.Output()
	if err != nil {
		return false, nil
	}
	lines := strings.Split(string(out), "\n")
	if lines[0] != "sat" {
		panic("not satisfiable, but no error")
	}
	for i := 0; i < len(results); i++ {
		results[i], _ = strconv.Atoi(lines[i+1])
	}
	return true, results
}

func validate(program []uint8) {
	size := len(program)
	outs := 0
	for i := 0; i < size-2; i += 2 {
		if program[i] == uint8(Out) {
			outs += 1
		}
		if program[i] == uint8(Jnz) {
			panic(fmt.Sprintf("jnz at %d", i))
		}
	}
	if outs != 1 {
		panic("too many outs")
	}
	if program[size-4] != uint8(Out) {
		panic("penultimate instruction should be out")
	}
	if program[size-3] < 4 && program[size-3] >= 7 {
		panic("operand to out should be a register")
	}
	if program[size-2] != uint8(Jnz) {
		panic("doesn't end with jnz")
	}
	if program[size-1] != 0 {
		panic(fmt.Sprintf("ends with jnz %d instead of 0", program[size-1]))
	}
}

func opToRegName(operand uint8) rune {
	switch operand {
	case 0:
		return '0'
	case 1:
		return '1'
	case 2:
		return '2'
	case 3:
		return '3'
	case 4:
		return 'a'
	case 5:
		return 'b'
	case 6:
		return 'c'
	}
	panic("invalid operand: not a register")
}

func opToRegIdx(operand uint8) int {
	switch operand {
	case 4:
		return 0
	case 5:
		return 1
	case 6:
		return 2
	}
	panic("invalid operand: not a register")
}

func regIdxToName(idx int) rune {
	switch idx {
	case 0:
		return 'a'
	case 1:
		return 'b'
	case 2:
		return 'c'
	}
	panic("invalid register index")
}

func declareConst(script *[]string, name string) {
	*script = append(*script, fmt.Sprintf("(declare-const %s (_ BitVec 64))", name))
}

func divToZ3(operand uint8, dstIdx int, rv *[3]int) []string {
	if dstIdx < 0 || dstIdx > 2 {
		panic("invalid destination")
	}
	dstName := regIdxToName(dstIdx)
	script := make([]string, 0, 4)
	numerator := fmt.Sprintf("a%d", rv[0])
	result := fmt.Sprintf("%c%d", dstName, rv[dstIdx]+1)

	// result
	declareConst(&script, result)
	// calculation
	var denominator string
	if operand <= 3 {
		// literal
		denominator = fmt.Sprintf("lit%d", operand)
	} else {
		// register
		denominator = fmt.Sprintf("%c%d", opToRegName(operand), rv[opToRegIdx(operand)])
	}
	script = append(script, fmt.Sprintf("(assert (= %s (bvlshr %s %s)))", result, numerator, denominator))
	// update written register
	rv[dstIdx]++
	return script
}

func (op opcode) disassemble(operand uint8) string {
	switch op {
	case Adv:
		return fmt.Sprintf("adv %c", opToRegName(operand))
	case Bdv:
		return fmt.Sprintf("bdv %c", opToRegName(operand))
	case Cdv:
		return fmt.Sprintf("cdv %c", opToRegName(operand))
	case Bxl:
		return fmt.Sprintf("bxl %d", operand)
	case Bst:
		return fmt.Sprintf("bst %c", opToRegName(operand))
	case Bxc:
		return fmt.Sprintf("bxc %d", operand)
	case Jnz:
		return fmt.Sprintf("jnz %d", operand)
	case Out:
		return fmt.Sprintf("out %c", opToRegName(operand))
	}
	panic("invalid op")
}

func addComment(script *[]string, comment string) {
	*script = append(*script, "; "+comment)
}

func opToZ3(op opcode, operand uint8, rv *[3]int) []string {
	script := make([]string, 0)
	addComment(&script, op.disassemble(operand))
	switch op {
	case Adv:
		return divToZ3(operand, 0, rv)
	case Bdv:
		return divToZ3(operand, 1, rv)
	case Cdv:
		return divToZ3(operand, 2, rv)
	case Bxl:
		// B = B^literal
		result := fmt.Sprintf("b%d", rv[1]+1)
		reg := fmt.Sprintf("b%d", rv[1])
		declareConst(&script, result)
		script = append(script, fmt.Sprintf("(assert (= %s (bvxor lit%d %s)))", result, operand, reg))
		// wrote to B
		rv[1]++
		return script
	case Bst:
		// B = combo % 8
		result := fmt.Sprintf("b%d", rv[1]+1)
		declareConst(&script, result)
		if operand <= 3 {
			// literal
			script = append(script, fmt.Sprintf("(assert (= %s (bvand lit%d lit7)))", result, operand))
		} else {
			// register
			reg := fmt.Sprintf("%c%d", opToRegName(operand), rv[opToRegIdx(operand)])
			script = append(script, fmt.Sprintf("(assert (= %s (bvand %s lit7)))", result, reg))
		}
		// wrote to B
		rv[1]++
		return script
	case Bxc:
		// B = B^C
		result := fmt.Sprintf("b%d", rv[1]+1)
		declareConst(&script, result)
		reg1 := fmt.Sprintf("b%d", rv[1])
		reg2 := fmt.Sprintf("c%d", rv[2])
		script = append(script, fmt.Sprintf("(assert (= %s (bvxor %s %s)))", result, reg1, reg2))
		// wrote to B
		rv[1]++
		return script
	}
	panic("invalid op")
}

func part2z3(program []uint8) {
	// assume program ends with jnz 0
	validate(program)

	rv := [3]int{0, 0, 0}
	var script []string
	addComment(&script, "initial registers")
	declareConst(&script, "a0")
	declareConst(&script, "b0")
	declareConst(&script, "c0")
	addComment(&script, "literals")
	for i := 0; i < 8; i++ {
		script = append(script, fmt.Sprintf("(declare-const lit%d (_ BitVec 64))\n(assert (= lit%d ((_ int2bv 64) %d)))", i, i, i))
	}

	size := len(program)
	outputRegName := opToRegName(program[size-3])
	outputRegIdx := program[size-3] - 4
	for i := 0; i < size; i++ {
		addComment(&script, fmt.Sprintf("iteration %d", i))
		for ip := 0; ip < size-4; ip += 2 {
			script = append(script, opToZ3(opcode(program[ip]), program[ip+1], &rv)...)
		}
		// assert output
		script = append(script, fmt.Sprintf("(assert (= (bvand %c%d lit7) lit%d))", outputRegName, rv[outputRegIdx], program[i]))
		// assert continuation or halt
		if i == size-1 {
			// assert halt
			script = append(script, fmt.Sprintf("(assert (= a%d lit%d))", rv[0], 0))
		} else {
			// assert continuation
			script = append(script, fmt.Sprintf("(assert (bvugt a%d lit%d))", rv[0], 0))
		}
	}

	addComment(&script, "solve")
	script = append(script, "(minimize (bv2int a0))")
	solved, results := z3(strings.Join(script, "\n"), "(bv2int a0)")
	if solved {
		fmt.Println("Part 2:", results[0])
	} else {
		fmt.Println("Part 2: unsolvable")
	}
}

func disassemble(program []uint8) {
	for i := 0; i < len(program); i += 2 {
		fmt.Println(opcode(program[i]).disassemble(program[i+1]))
	}
}

func main() {
	regs, prog := readInput()
	fmt.Println("Input:", regs, prog)
	disassemble(prog)
	part1(regs, prog)
	part2z3(prog)
}
