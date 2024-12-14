package main

import (
	"bufio"
	"fmt"
	"os"
	"os/exec"
	"strconv"
	"strings"
)

type coord struct {
	x, y int
}

type claw struct {
	a, b, prize coord
}

type solution struct {
	a, b, tokens int
}

func (c claw) String() string {
	return fmt.Sprintf("Button A: X+%d, Y+%d\nButton B: X+%d, Y+%d\nPrize: X=%d, Y=%d\n", c.a.x, c.a.y, c.b.x, c.b.y, c.prize.x, c.prize.y)
}

func (c claw) validate() {
	if c.a.x <= 0 || c.a.y <= 0 || c.b.x <= 0 || c.b.y <= 0 || c.prize.x <= 0 || c.prize.y <= 0 {
		panic("bad claw!")
	}
}

func readInput() []claw {
	scanner := bufio.NewScanner(os.Stdin)
	claws := make([]claw, 0, 10)
	for scanner.Scan() {
		lineButtonA := scanner.Text()
		scanner.Scan()
		lineButtonB := scanner.Text()
		scanner.Scan()
		linePrize := scanner.Text()
		scanner.Scan() // empty
		var claw claw
		fmt.Sscanf(lineButtonA, "Button A: X+%d, Y+%d", &claw.a.x, &claw.a.y)
		fmt.Sscanf(lineButtonB, "Button B: X+%d, Y+%d", &claw.b.x, &claw.b.y)
		fmt.Sscanf(linePrize, "Prize: X=%d, Y=%d", &claw.prize.x, &claw.prize.y)
		claw.validate()
		claws = append(claws, claw)
	}

	return claws
}

func (c claw) solve() (bool, solution) {
	var result solution
	script := fmt.Sprintf(`(declare-const ax Int) ; button A X
(declare-const ay Int) ; button A Y
(declare-const bx Int) ; button B X
(declare-const by Int) ; button B Y
(declare-const px Int) ; prize X
(declare-const py Int) ; prize Y
(declare-const pa Int) ; presses of A
(declare-const pb Int) ; presses of B
(declare-const tokens Int)
; claw parameters
(assert (= ax %d))
(assert (= ay %d))
(assert (= bx %d))
(assert (= by %d))
(assert (= px %d))
(assert (= py %d))
; press no more than 100 times
;(assert (<= pa 100))
;(assert (<= pb 100))
; the actual stuff
(assert (= (+ (* ax pa) (* bx pb)) px))
(assert (= (+ (* ay pa) (* by pb)) py))
(assert (= tokens (+ (* 3 pa) pb)))
(minimize tokens)
(check-sat)
(eval pa)
(eval pb)
(eval tokens)`, c.a.x, c.a.y, c.b.x, c.b.y, c.prize.x, c.prize.y)
	cmd := exec.Command("z3", "-in")
	cmd.Stdin = strings.NewReader(script)
	out, err := cmd.Output()
	if err == nil {
		lines := strings.Split(string(out), "\n")
		if lines[0] != "sat" {
			panic("not satisfiable, but no error")
		}
		result.a, _ = strconv.Atoi(lines[1])
		result.b, _ = strconv.Atoi(lines[2])
		result.tokens, _ = strconv.Atoi(lines[3])
	}
	return err != nil, result
}

func part1(claws []claw) {
	tokens := 0
	for _, claw := range claws {
		_, solution := claw.solve()
		tokens += solution.tokens
	}
	fmt.Println("Part 1", tokens)
}

func part2(claws []claw) {
	tokens := 0
	for _, claw := range claws {
		claw.prize.x += 10000000000000
		claw.prize.y += 10000000000000
		_, solution := claw.solve()
		tokens += solution.tokens
	}
	fmt.Println("Part 2", tokens)
}

func main() {
	claws := readInput()
	part1(claws)
	part2(claws)
}
