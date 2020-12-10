package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
)

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	re := regexp.MustCompile(`(\d+)-(\d+) (\w): (\w+)`)
	valid := 0
	for scanner.Scan() {
		line := re.FindStringSubmatch(scanner.Text())
		if line == nil {
			fmt.Println("WARNING: Unmatching line")
			continue
		}
		min, err := strconv.Atoi(line[1])
		if err != nil {
			panic(err)
		}
		max, err := strconv.Atoi(line[2])
		if err != nil {
			panic(err)
		}
		c := byte(line[3][0])
		pass := line[4]
		if (pass[min-1] == c) != (pass[max-1] == c) {
			valid++
		}
	}
	fmt.Println(valid)
}
