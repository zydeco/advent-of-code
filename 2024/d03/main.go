package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func readInput() string {
	scanner := bufio.NewScanner(os.Stdin)
	input := ""
	for scanner.Scan() {
		input += scanner.Text()
	}
	return input
}

func doMultiplications(input *string) int64 {
	regex, err := regexp.Compile(`mul\((\d{1,3}),(\d{1,3})\)`)
	if err != nil {
		panic(err)
	}
	matches := regex.FindAllStringSubmatch(*input, -1)
	var result int64 = 0
	for i := range matches {
		match := matches[i]
		a, _ := strconv.ParseInt(match[1], 10, 0)
		b, _ := strconv.ParseInt(match[2], 10, 0)
		result += a * b
	}
	return result
}

func matchToNamed(match []string, names []string) map[string]string {
	matchMap := make(map[string]string)
	for i, name := range names {
		if i != 0 && match[i] != "" {
			matchMap[name] = match[i]
		}
	}
	return matchMap
}

func doPart2(input *string) int64 {
	regex := regexp.MustCompile(`(?:(?P<op>mul)\((?P<arg1>\d{1,3}),(?P<arg2>\d{1,3})\))|(?:(?P<op>do)\(\))|(?:(?P<op>don't)\(\))`)
	matches := regex.FindAllStringSubmatch(*input, -1)
	var result int64 = 0
	mulling := true
	for i := range matches {
		match := matchToNamed(matches[i], regex.SubexpNames())
		instr := match["op"]
		switch instr {
		case "mul":
			if mulling {
				a, _ := strconv.ParseInt(match["arg1"], 10, 0)
				b, _ := strconv.ParseInt(match["arg2"], 10, 0)
				result += a * b
			}
		case "do":
			mulling = true
		case "don't":
			mulling = false
		}
	}
	return result
}

func main() {
	input := readInput()
	result := doMultiplications(&input)
	fmt.Println("Result:", result)
	part2 := doPart2(&input)
	fmt.Println("Part 2:", part2)
}
