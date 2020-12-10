package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	report := make(map[int]bool)
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		i, err := strconv.Atoi(scanner.Text())
		if err != nil {
			panic(err)
		}
		report[i] = true
	}
	if scerr := scanner.Err(); scerr != nil {
		panic(scerr)
	}
	for i := range report {
		p := 2020 - i
		if _, ok := report[p]; ok {
			fmt.Printf("%d + %d = 2020\n%d * %d = %d\n", i, p, i, p, i*p)
			return
		}
	}
}
