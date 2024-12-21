package main

import (
	"bufio"
	"fmt"
	"os"
)

type code []KeypadButton

type KeypadButton int8

const (
	KP0 KeypadButton = iota
	KP1
	KP2
	KP3
	KP4
	KP5
	KP6
	KP7
	KP8
	KP9
	KPA
)

func (k KeypadButton) String() string {
	if k == KPA {
		return "A"
	} else {
		return string('0' + k)
	}
}

func (c code) String() string {
	s := ""
	for _, button := range c {
		s += button.String()
	}
	return s
}

func (c code) numericValue() int {
	value := 0
	for _, b := range c {
		if b == KPA {
			continue
		}
		value = value*10 + int(b)
	}
	return value
}

func parseButton(c rune) KeypadButton {
	switch c {
	case '0':
		return KP0
	case '1':
		return KP1
	case '2':
		return KP2
	case '3':
		return KP3
	case '4':
		return KP4
	case '5':
		return KP5
	case '6':
		return KP6
	case '7':
		return KP7
	case '8':
		return KP8
	case '9':
		return KP9
	case 'A':
		return KPA
	default:
		panic("Invalid keypad button!")
	}
}

func parseCode(s string) code {
	code := make(code, 0, 4)
	for _, v := range s {
		code = append(code, parseButton(v))
	}
	return code
}

func readInput() []code {
	codes := make([]code, 0, 5)
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		codes = append(codes, parseCode(scanner.Text()))
	}

	return codes
}

type DPadButton int8

const (
	DPUp DPadButton = iota
	DPDown
	DPLeft
	DPRight
	DPActivate
)

func (b DPadButton) String() string {
	switch b {
	case DPUp:
		return "^"
	case DPDown:
		return "v"
	case DPLeft:
		return "<"
	case DPRight:
		return ">"
	case DPActivate:
		return "A"
	default:
		panic("Invalid DPadButton")
	}
}

func repeated[E any](value E, times int) []E {
	slice := make([]E, times)
	for i := 0; i < times; i++ {
		slice[i] = value
	}
	return slice
}

func moveTo(from, to int, decrease, increase DPadButton) []DPadButton {
	direction := decrease
	count := from - to
	if to > from {
		direction = increase
		count = to - from
	}
	if to == from {
		return []DPadButton{}
	}
	return repeated(direction, count)
}

func moveToRow(from, to int) []DPadButton {
	return moveTo(from, to, DPUp, DPDown)
}

func moveToCol(from, to int) []DPadButton {
	return moveTo(from, to, DPLeft, DPRight)
}

func makeMoves(fromRow, fromCol, toRow, toCol, avoidRow, avoidCol int) DPadSequence {
	moves := make([]DPadButton, 0, 4)

	if fromRow == avoidRow && toCol == avoidCol {
		moves = append(moves, moveToRow(fromRow, toRow)...)
		moves = append(moves, moveToCol(fromCol, toCol)...)
	} else if toRow == avoidRow && fromCol == avoidCol {
		moves = append(moves, moveToCol(fromCol, toCol)...)
		moves = append(moves, moveToRow(fromRow, toRow)...)
	} else {
		// doesn't pass empty space
		if toCol < fromCol {
			// must move left
			moves = append(moves, moveToCol(fromCol, toCol)...)
			moves = append(moves, moveToRow(fromRow, toRow)...)
		} else if toRow > fromRow {
			// must move down
			moves = append(moves, moveToRow(fromRow, toRow)...)
			moves = append(moves, moveToCol(fromCol, toCol)...)
		} else {
			// anything else
			moves = append(moves, moveToCol(fromCol, toCol)...)
			moves = append(moves, moveToRow(fromRow, toRow)...)
		}
	}

	// activate
	moves = append(moves, DPActivate)
	return moves
}

func keypadMoves(from, to KeypadButton) DPadSequence {
	buttonRow := [...]int{3, 2, 2, 2, 1, 1, 1, 0, 0, 0, 3}
	buttonCol := [...]int{1, 0, 1, 2, 0, 1, 2, 0, 1, 2, 2}
	fromRow, fromCol := buttonRow[from], buttonCol[from]
	toRow, toCol := buttonRow[to], buttonCol[to]
	return makeMoves(fromRow, fromCol, toRow, toCol, 3, 0)
}

func dpadMoves(from, to DPadButton) DPadSequence {
	buttonRow := [...]int{0, 1, 1, 1, 0}
	buttonCol := [...]int{1, 1, 0, 2, 2}
	fromRow, fromCol := buttonRow[from], buttonCol[from]
	toRow, toCol := buttonRow[to], buttonCol[to]
	return makeMoves(fromRow, fromCol, toRow, toCol, 0, 0)
}

func (c code) keypadMoves() DPadSequence {
	currentButton := KPA
	moves := make(DPadSequence, 0, 4)
	for _, nextButton := range c {
		moves = append(moves, keypadMoves(currentButton, nextButton)...)
		currentButton = nextButton
	}
	return moves
}

type DPadSequence []DPadButton

func (s DPadSequence) dpadMoves() DPadSequence {
	currentButton := DPActivate
	moves := make(DPadSequence, 0, 4)
	for _, nextButton := range s {
		moves = append(moves, dpadMoves(currentButton, nextButton)...)
		currentButton = nextButton
	}
	return moves
}

func (s DPadSequence) String() string {
	str := ""
	for _, b := range s {
		str += b.String()
	}
	return str
}

func part1(codes []code) {
	complexity := 0
	for _, code := range codes {
		complexity += len(code.keypadMoves().dpadMoves().dpadMoves()) * code.numericValue()
	}
	fmt.Println("Complexity", complexity)
}

func main() {
	codes := readInput()
	fmt.Println("Input:", codes)
	part1(codes)
}
