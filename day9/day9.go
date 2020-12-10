package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	preamble := 25
	previous := make([]int64, preamble)
	i := 0
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		n, err := strconv.ParseInt(scanner.Text(), 10, 64)
		if err != nil {
			panic(err)
		}
		found := false
		if i < 25 {
			found = true
		}
		for i := range previous {
			for j := range previous {
				if i != j && previous[i]+previous[j] == n {
					fmt.Println(previous[i], "+", previous[j], "=", n)
					found = true
				}
			}
		}
		if found == false {
			fmt.Println("NO MATCH FOR", n)
		}
		previous[i%preamble] = n
		i++
	}
	if scerr := scanner.Err(); scerr != nil {
		panic(scerr)
	}
}
