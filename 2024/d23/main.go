package main

import (
	"bufio"
	"fmt"
	"os"
	"slices"
)

type computer [2]byte

type connection [2]computer

type party []computer

func (c computer) String() string {
	return fmt.Sprintf("%c%c", c[0], c[1])
}

func (c connection) String() string {
	return fmt.Sprintf("%s-%s", c[0], c[1])
}

func (p party) String() string {
	s := p[0].String()
	for i := 1; i < len(p); i++ {
		s += fmt.Sprintf(",%s", p[i])
	}
	return s
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
	return slices.SortedFunc(slices.Values(p[:]), compareComputers)
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
	for i := 0; i < len(p); i++ {
		for j := i + 1; j < len(p); j++ {
			conn := connection{p[i], p[j]}.sorted()
			if !connections[conn] {
				return false
			}
		}
	}
	return true
}

func (p party) isConnectedTo(c computer, connections map[connection]bool) bool {
	// assume party is already connected
	for i := 0; i < len(p); i++ {
		conn := connection{p[i], c}.sorted()
		if !connections[conn] {
			return false
		}
	}
	return true
}

func (p party) maybeHasChief() bool {
	// any computer starts with 't'
	has := false
	for i := 0; i < len(p); i++ {
		if p[i][0] == 't' {
			has = true
		}
	}
	return has
}

func part1(computers []computer, connections map[connection]bool) {
	matchingParties := make(map[[3]computer]bool)
	for conn := range connections {
		for _, c := range computers {
			if c == conn[0] || c == conn[1] {
				continue
			}
			party := party{conn[0], conn[1], c}.sorted()
			if party.isConnected(connections) && party.maybeHasChief() {
				matchingParties[[3]computer(party)] = true
			}
		}
	}
	fmt.Printf("Part 1: %d\n", len(matchingParties))
}

func (c computer) connectedParty(connections map[connection]bool) party {
	party := make(party, 0, 16)
	party = append(party, c)
	for conn := range connections {
		if conn[0] == c && party.isConnectedTo(conn[1], connections) {
			party = append(party, conn[1])
		} else if conn[1] == c && party.isConnectedTo(conn[0], connections) {
			party = append(party, conn[0])
		}
	}
	slices.SortFunc(party, compareComputers)
	return party
}

func part2(computers []computer, connections map[connection]bool) {
	largestParty := make(party, 0)
	for _, c := range computers {
		party := c.connectedParty(connections)
		if len(party) > len(largestParty) {
			largestParty = party
		}
	}
	fmt.Printf("Part 2: %s (%d)\n", largestParty, len(largestParty))
}

func main() {
	input := readInput()
	computers, connections := parseConnections(input)
	part1(computers, connections)
	part2(computers, connections)
}
