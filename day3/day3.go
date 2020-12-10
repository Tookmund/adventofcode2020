package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	trees := make([]string, 0)
	for scanner.Scan() {
		trees = append(trees, scanner.Text())
	}
	if scerr := scanner.Err(); scerr != nil {
		panic(scerr)
	}
	v := 0
	dv := 1
	h := 0
	dh := 3

	numtrees := 0
	for v < len(trees) {
		if trees[v][h%len(trees[v])] == '#' {
			numtrees++
		}
		v += dv
		h += dh
	}
	fmt.Println(numtrees)
}
