package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func main() {
	report := make([]int, 0)
	scanner := bufio.NewScanner(os.Stdin)
	for scanner.Scan() {
		i, err := strconv.Atoi(scanner.Text())
		if err != nil {
			panic(err)
		}
		report = append(report, i)
	}
	if scerr := scanner.Err(); scerr != nil {
		panic(scerr)
	}
	for i := range report {
		for j := i; j < len(report); j++ {
			for k := j; k < len(report); k++ {
				if report[i]+report[j]+report[k] == 2020 {
					fmt.Printf("%d + %d + %d = 2020\n%d * %d * %d = %d\n", report[i], report[j], report[k], report[i], report[j], report[k], report[i]*report[j]*report[k])
					return
				}
			}
		}
	}
}
