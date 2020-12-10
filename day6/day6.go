package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type grouptype map[string]bool

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	groups := make([]grouptype, 0)
	groups = addgroup(groups)
	i := 0
	for scanner.Scan() {
		line := strings.Split(scanner.Text(), "")
		fmt.Println(line, len(line))
		if len(line) == 0 {
			groups = addgroup(groups)
			i++
		}
		for _, v := range line {
			groups[i][v] = true
		}
	}
	fmt.Println(groups)
	if scerr := scanner.Err(); scerr != nil {
		panic(scerr)
	}
	total := 0
	for _, group := range groups {
		for range group {
			total++
		}
	}
	fmt.Println(total)
}
func addgroup(g []grouptype) []grouptype {
	n := make(grouptype)
	return append(g, n)
}
