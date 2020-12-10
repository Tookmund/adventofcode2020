package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
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
	hgtreg := regexp.MustCompile(`^(\d+)(cm|in)$`)
	hclreg := regexp.MustCompile(`^#[0-9a-f]{6}$`)
	eclreg := regexp.MustCompile(`^(amb|blu|brn|gry|grn|hzl|oth)$`)
	pidreg := regexp.MustCompile(`^\d{9}$`)
	valid := 0
	for _, pass := range passports {
		numreq := 0
		numvalid := 0

		for _, req := range required {
			if v, ok := pass[req]; ok {
				numreq++
				switch req {
				case "byr":
					iv, err := strconv.Atoi(v)
					if err != nil {
						continue
					}
					if len(v) == 4 && iv >= 1920 && iv <= 2002 {
						numvalid++
					}
				case "iyr":
					iv, err := strconv.Atoi(v)
					if err != nil {
						continue
					}
					if len(v) == 4 && iv >= 2010 && iv <= 2020 {
						numvalid++
					}
				case "eyr":
					iv, err := strconv.Atoi(v)
					if err != nil {
						continue
					}
					if len(v) == 4 && iv >= 2020 && iv <= 2030 {
						numvalid++
					}
				case "hgt":
					h := hgtreg.FindStringSubmatch(v)
					if h != nil {
						unit := string(h[2])
						size, err := strconv.Atoi(h[1])
						if err != nil {
							continue
						}
						if unit == "cm" && size >= 150 && size <= 193 {
							numvalid++
						} else if unit == "in" && size >= 59 && size <= 76 {
							numvalid++
						}
					}
				case "hcl":
					if hclreg.MatchString(v) {
						numvalid++
					}
				case "ecl":
					if eclreg.MatchString(v) {
						numvalid++
					}
				case "pid":
					if pidreg.MatchString(v) {
						numvalid++
					}
				}
			}
		}
		if numreq == len(required) && numvalid == len(required) {
			valid++
		}
	}
	fmt.Println(valid)
}

func addentry(p []map[string]string) []map[string]string {
	n := make(map[string]string)
	return append(p, n)
}
