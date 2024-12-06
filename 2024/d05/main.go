package main

import (
	"bufio"
	"fmt"
	"os"
	"sort"
	"strconv"
	"strings"
)

type rule [2]int
type update []int

func (u *update) hasRepeatedPages() bool {
	found := make(map[int]bool)
	for _, page := range *u {
		if found[page] {
			return true
		}
		found[page] = true
	}
	return false
}

func (u *update) isValid(rules *[]rule) bool {
	order := make(map[int]int)
	// map order
	for index, page := range *u {
		order[page] = index + 1 // go map returns zero if not present
	}
	// apply rules
	for _, rule := range *rules {
		former, latter := order[rule[0]], order[rule[1]]
		if former != 0 && latter != 0 && former > latter {
			return false
		}
	}
	return true
}

func (u *update) middle() int {
	return (*u)[len(*u)/2]
}

type ruleLookupTable map[int]int

func makeLookupTable(r *[]rule) ruleLookupTable {
	table := make(map[int]int)
	// if x before y, table[100x+y] = -1
	for _, rule := range *r {
		table[rule[0]*100+rule[1]] = -1
		table[rule[1]*100+rule[0]] = 1
	}
	return table
}

func sortedUpdate(u *update, rules *ruleLookupTable) update {
	sortedUpdate := make(update, len(*u))
	copy(sortedUpdate, *u)
	sort.SliceStable(sortedUpdate, func(i, j int) bool {
		a, b := sortedUpdate[i], sortedUpdate[j]
		return (*rules)[100*a+b] == -1 || (*rules)[100*b+a] == 1
	})
	return sortedUpdate
}

func readInput() ([]rule, []update) {
	scanner := bufio.NewScanner(os.Stdin)
	rules := make([]rule, 0, 64)
	updates := make([]update, 0, 64)

	// read rules
	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			break
		}
		var a, b int
		_, err := fmt.Sscanf(line, "%d|%d", &a, &b)
		if err != nil {
			panic(err)
		}
		rules = append(rules, [2]int{a, b})
	}

	// read updates
	for scanner.Scan() {
		line := scanner.Text()
		update := make([]int, 0, 16)
		words := strings.Split(line, ",")
		for _, word := range words {
			num, err := strconv.ParseInt(word, 10, 0)
			if err != nil {
				panic("Invalid input")
			}
			update = append(update, int(num))
		}
		updates = append(updates, update)
	}
	return rules, updates
}

func main() {
	rules, updates := readInput()
	// sanity check
	for _, update := range updates {
		if update.hasRepeatedPages() {
			panic("Invalid update!")
		}
	}
	// part 1
	result := 0
	invalidUpdates := make([]update, 0, len(updates)/2)
	for _, update := range updates {
		if update.isValid(&rules) {
			result += update.middle()
		} else {
			invalidUpdates = append(invalidUpdates, update)
		}
	}
	fmt.Println("Part 1", result)

	// part 2
	rlt := makeLookupTable(&rules)
	result2 := 0
	for _, update := range invalidUpdates {
		sorted := sortedUpdate(&update, &rlt)
		result2 += sorted.middle()
	}
	fmt.Println("Part 2", result2)
}
