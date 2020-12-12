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
	all := make([]int64, 0)
	var invalid int64 = -1
	i := 0
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		n, err := strconv.ParseInt(scanner.Text(), 10, 64)
		if err != nil {
			panic(err)
		}
		found := false
		if i < preamble {
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
		if found == false && invalid == -1 {
			fmt.Println("Invalid:", n)
			invalid = n
		}
		previous[i%preamble] = n
		i++
		all = append(all, n);
	}
	if scerr := scanner.Err(); scerr != nil {
		panic(scerr)
	}

	for s := 0; s < len(all); s++ {
		runtot := all[s];
		smallest := runtot;
		largest := runtot;
		for e := s+1; e < len(all); e++ {
			runtot += all[e]
			if all[e] < smallest {
				smallest = all[e]
			} else if all[e] > largest {
				largest = all[e]
			}
			if runtot == invalid {
				fmt.Println("Smallest:", smallest, "\nLargest:", largest,
					"\nAdded:", smallest+largest);
			}
		}
	}
}
