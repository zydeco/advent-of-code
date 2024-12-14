package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

type coord struct {
	x, y int
}

type robot struct {
	position, velocity coord
}

func (c coord) mul(times int) coord {
	return coord{x: c.x * times, y: c.y * times}
}

func (c coord) add(c2 coord) coord {
	return coord{x: c.x + c2.x, y: c.y + c2.y}
}

func (c coord) mod(x, y int) coord {
	return coord{x: modulo(c.x, x), y: modulo(c.y, y)}
}

func modulo(x int, m int) int {
	x %= m
	if x < 0 {
		x += m
	}
	return x
}

func (r robot) move(times, width, height int) robot {
	return robot{
		position: r.position.add(r.velocity.mul(times)).mod(width, height),
		velocity: r.velocity,
	}
}

func (r robot) String() string {
	return fmt.Sprintf("p=%d,%d v=%d,%d", r.position.x, r.position.y, r.velocity.x, r.velocity.y)
}

func readInput() []robot {
	scanner := bufio.NewScanner(os.Stdin)
	robots := make([]robot, 0, 10)

	for scanner.Scan() {
		line := scanner.Text()
		if line == "" {
			panic("empty line")
		}
		var r robot
		scanned, _ := fmt.Sscanf(line, "p=%d,%d v=%d,%d", &r.position.x, &r.position.y, &r.velocity.x, &r.velocity.y)
		if scanned != 4 {
			panic("invalid robot: " + line)
		}
		robots = append(robots, r)
	}

	return robots
}

func safetyFactor(robots []robot, width, height int) int {
	var q1, q2, q3, q4 = countPerQuadrant(robots, width, height)
	return q1 * q2 * q3 * q4
}

func countPerQuadrant(robots []robot, width, height int) (int, int, int, int) {
	var q1, q2, q3, q4 int
	for _, r := range robots {
		if r.position.x < width/2 {
			if r.position.y < height/2 {
				q1 += 1
			} else if r.position.y > height/2 {
				q2 += 1
			}
		} else if r.position.x > width/2 {
			if r.position.y < height/2 {
				q3 += 1
			} else if r.position.y > height/2 {
				q4 += 1
			}
		}
	}
	return q1, q2, q3, q4
}

func robotsPerPosition(robots []robot) map[coord]int {
	count := make(map[coord]int)
	for _, r := range robots {
		count[r.position] += 1
	}
	return count
}

func printMap(robots []robot, width, height int, quadrants bool) {
	count := robotsPerPosition(robots)
	for y := 0; y < height; y++ {
		line := ""
		for x := 0; x < width; x++ {
			if quadrants && (x == width/2 || y == height/2) {
				line += " "
				continue
			}
			xy := coord{x: x, y: y}
			c := count[xy]
			if c == 0 {
				line += "."
			} else if c <= 36 {
				line += strconv.FormatInt(int64(c), 36)
			} else {
				line += "*"
			}
		}
		fmt.Println(line)
	}
}

func mapSize(robots []robot) (int, int) {
	if len(robots) < 20 {
		return 11, 7
	}
	return 101, 103
}

func part1(robots []robot) {
	width, height := mapSize(robots)
	fmt.Println(robots)
	printMap(robots, width, height, false)

	endState := make([]robot, len(robots))
	for i, robot := range robots {
		endState[i] = robot.move(100, width, height)
	}
	fmt.Println("After:")
	printMap(endState, width, height, true)
	fmt.Println("Part 1", safetyFactor(endState, width, height))
}

func possibleXmasTree(robots []robot, width, height int) bool {
	var q1, q2, q3, q4 = countPerQuadrant(robots, width, height)
	return q1 > (q2+q3+q4) ||
		q2 > (q1+q3+q4) ||
		q3 > (q1+q2+q4) ||
		q4 > (q1+q2+q3)
}

func part2(initialRobots []robot) {
	width, height := mapSize(initialRobots)
	robots := make([]robot, len(initialRobots))
	copy(robots, initialRobots)
	for step := 0; step < 10000000; step++ {
		for i, robot := range robots {
			robots[i] = robot.move(1, width, height)
		}
		if possibleXmasTree(robots, width, height) {
			printMap(robots, width, height, false)
			fmt.Println("Step", step+1)
			return
		}
	}
}

func main() {
	robots := readInput()
	part1(robots)
	part2(robots)
}
