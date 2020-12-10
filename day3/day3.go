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

	slopes := [][]int{[]int{1, 1}, []int{3, 1}, []int{5, 1}, []int{7, 1}, []int{1, 2}}

	multtrees := 1

	for i := range slopes {
		v := 0
		dv := slopes[i][1]
		h := 0
		dh := slopes[i][0]

		numtrees := 0
		for v < len(trees) {
			if trees[v][h%len(trees[v])] == '#' {
				numtrees++
			}
			v += dv
			h += dh
		}
		multtrees *= numtrees
	}
	fmt.Println(multtrees)
}
