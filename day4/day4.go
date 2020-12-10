package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func main() {
	passports := make([]map[string]string, 0)
	passports = addentry(passports)
	scanner := bufio.NewScanner(os.Stdin)
	i := 0
	for scanner.Scan() {
		line := strings.Split(scanner.Text(), " ")
		if len(line) == 1 && line[0] == "" {
			passports = addentry(passports)
			i++
		}
		for _, entry := range line {
			kv := strings.Split(entry, ":")
			if kv[0] == "" {
				break
			}
			passports[i][kv[0]] = kv[1]
		}
	}

	required := []string{"byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"}
	valid := 0
	for _, pass := range passports {
		numreq := 0
		for _, req := range required {
			if _, ok := pass[req]; ok  {
				numreq++
			}
		}
		if numreq == len(required) {
			valid++
		}
	}
	fmt.Println(valid)
}

func addentry(p []map[string]string) []map[string]string {
	n := make(map[string]string)
	return append(p, n)
}
