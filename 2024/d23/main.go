package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
)

type computer [2]byte

type connection [2]computer

type party [3]computer

func (c computer) String() string {
	return fmt.Sprintf("%c%c", c[0], c[1])
}

func (c connection) String() string {
	return fmt.Sprintf("%s-%s", c[0], c[1])
}

func (p party) String() string {
	return fmt.Sprintf("%s,%s,%s", p[0], p[1], p[2])
}

func compareComputers(c1, c2 computer) int {
	if c1[0] == c2[0] {
		return int(c1[1]) - int(c2[1])
	}
	return int(c1[0]) - int(c2[0])
}

func (c connection) sorted() connection {
	sorted := slices.SortedFunc(slices.Values(c[:]), compareComputers)
	return connection{sorted[0], sorted[1]}
}

func (p party) sorted() party {
	sorted := slices.SortedFunc(slices.Values(p[:]), compareComputers)
	return party{sorted[0], sorted[1], sorted[2]}
}

func readInput() []connection {
	connections := make([]connection, 0, 1)
	scanner := bufio.NewScanner(os.Stdin)

	for scanner.Scan() {
		var c1s, c2s string
		_, err := fmt.Sscanf(scanner.Text(), "%2s-%2s", &c1s, &c2s)
		if err != nil {
			panic(err)
		}
		c1b := [2]byte{c1s[0], c1s[1]}
		c2b := [2]byte{c2s[0], c2s[1]}
		connections = append(connections, connection{c1b, c2b})
	}
	return connections
}

func parseConnections(connections []connection) ([]computer, map[connection]bool) {
	connMap := make(map[connection]bool)
	compMap := make(map[computer]bool)
	for _, conn := range connections {
		connMap[conn.sorted()] = true
		compMap[conn[0]] = true
		compMap[conn[1]] = true
	}
	compList := make([]computer, 0, len(compMap))
	for computer := range compMap {
		compList = append(compList, computer)
	}
	return compList, connMap
}

func (p party) isConnected(connections map[connection]bool) bool {
	conn1 := connection{p[0], p[1]}.sorted()
	conn2 := connection{p[1], p[2]}.sorted()
	conn3 := connection{p[2], p[0]}.sorted()
	return connections[conn1] && connections[conn2] && connections[conn3]
}

func (p party) maybeHasChief() bool {
	// any computer starts with 't'
	return p[0][0] == 't' || p[1][0] == 't' || p[2][0] == 't'
}

func part1(computers []computer, connections map[connection]bool) {
	parties := make([]party, 0)
	for i := 0; i < len(computers); i++ {
		for j := i + 1; j < len(computers); j++ {
			for k := j + 1; k < len(computers); k++ {
				party := party{computers[i], computers[j], computers[k]}.sorted()
				if party.isConnected(connections) && party.maybeHasChief() {
					parties = append(parties, party)
				}
			}
		}
	}
	fmt.Printf("Part 1: %d\n", len(parties))
}

func main() {
	input := readInput()
	computers, connections := parseConnections(input)
	fmt.Println("Input:", input)
	part1(computers, connections)
}
