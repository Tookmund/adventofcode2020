package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

type grouptype map[string]int

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	groups := make([]grouptype, 0)
	groups = addgroup(groups)
	i := 0
	for scanner.Scan() {
		line := strings.Split(scanner.Text(), "")
		fmt.Println(line)
		if len(line) == 0 {
			groups = addgroup(groups)
			i++
			continue
		}
		for _, v := range line {
			if _, ok := groups[i][v]; ok {
				groups[i][v] += 1
			} else {
				groups[i][v] = 1
			}
		}
		if _, ok := groups[i]["total"]; ok {
			groups[i]["total"] += 1
		} else {
			groups[i]["total"] = 1
		}
	}
	fmt.Println(groups)
	if scerr := scanner.Err(); scerr != nil {
		panic(scerr)
	}
	total := 0
	for _, group := range groups {
		for k, v := range group {
			if k != "total" && v == group["total"] {
				total++
			}
		}
	}
	fmt.Println(total)
}
func addgroup(g []grouptype) []grouptype {
	n := make(grouptype)
	return append(g, n)
}
