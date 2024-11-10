package main

import (
	"log"
	"math"
	"os"
	"slices"
	"strings"
)

type node struct {
	element, L, R string
}

func parse_nodes(node_lines []string) []node {
	nodes := []node{}

	for _, node_line := range node_lines {
		// fmt.Sprintf("node_line: %q", node_line)
		// split element and nodes
		first_split := strings.Split(node_line, "=")
		// get element
		element := strings.TrimSpace(first_split[0])
		// get nodes
		second_split := strings.Split(first_split[1], ",")
		left := strings.TrimPrefix(strings.TrimSpace(second_split[0]), "(")
		right := strings.TrimSuffix(strings.TrimSpace(second_split[1]), ")")

		nodes = append(nodes, node{element, left, right})
	}

	return nodes
}

func find_in_nodes(elem string, nodes []node) node {
	for _, node := range nodes {
		if elem == node.element {
			return node
		}
	}
	panic("node not found")
}

func solve(sequence []rune, nodes []node) int {
	current_step := sequence[0]
	current_node := find_in_nodes("AAA", nodes)
	steps := 0
	for {

		if current_node.element == "ZZZ" {
			return steps
		}

		switch current_step {
		case 'L':
			current_node = find_in_nodes(current_node.L, nodes)
			break
		case 'R':
			current_node = find_in_nodes(current_node.R, nodes)
			break
		default:
			log.Panicf("not a valid sequence %s", string(current_step))
		}

		steps += 1
		current_step = sequence[int(math.Mod(float64(steps), float64(len(sequence))))]
	}
}

func part_1(file_name string) int {
	contents, err := os.ReadFile(file_name)
	if err != nil {
		log.Panic(err)
	}

	lines := strings.Split(string(contents), "\n")

	sequence := []rune(strings.TrimSpace(lines[0]))
	node_lines := slices.DeleteFunc(lines[1:], func(str string) bool { return str == "" })

	nodes := parse_nodes(node_lines)

	return solve(sequence, nodes)
}

func main() {
	result := 0

	result = part_1("data/example.txt")
	log.Printf("Example 1 result: %d", result)
	log.Print("--------------------")
	if result != 2 {
		panic("failed example 1")
	}

	result = part_1("data/example2.txt")
	log.Printf("Example 2 result: %d", result)
	log.Print("--------------------")
	if result != 6 {
		panic("failed example 2")
	}

	result = part_1("data/data.txt")
	log.Printf("Part 1 result: %d", result)
	log.Print("--------------------")
	if result != 22411 {
		panic("failed part 1")
	}
}
