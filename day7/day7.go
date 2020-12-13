package main

import (
	"bufio"
	"fmt"
	"os"
	"regexp"
	"strconv"
	"strings"
)

type Bag struct {
	name  string
	count int
}

func main() {
	bagre := regexp.MustCompile(`(\d+) (\w+ \w+) bag(:?s)*`)
	scanner := bufio.NewScanner(os.Stdin)
	bags := make(map[string][]Bag, 0)
	for scanner.Scan() {
		line := strings.Split(scanner.Text(), "contain")
		line[1] = strings.TrimSuffix(line[1], ".")
		bag := strings.Replace(line[0], "bags", "", 1)
		bag = strings.TrimSpace(bag)
		contents := strings.Split(line[1], ", ")
		contents[0] = strings.TrimSpace(contents[0])
		if contents[0] == "no other bags" {
			bags[bag] = nil
			continue
		}
		bagcont := make([]Bag, 0, 2)
		for _, c := range contents {
			fmt.Println(bag, c)
			curbag := Bag{"", 0}
			m := bagre.FindStringSubmatch(c)
			var err error
			curbag.count, err = strconv.Atoi(m[1])
			if err != nil {
				panic(err)
			}
			curbag.name = m[2]
			bagcont = append(bagcont, curbag)
		}
		bags[bag] = bagcont
	}
	if scerr := scanner.Err(); scerr != nil {
		panic(scerr)
	}
	fmt.Printf("%d bags\n", len(bags))
	ways := make(map[string]int)
	ways["shiny gold"] = 1
	// We don't want use shiny gold directly
	total := 0
	for k, _ := range bags {
		r := getways(bags, ways, k)
		if r > 0 && k != "shiny gold" {
			total += 1
			fmt.Println("Using a", k, "bag")
		}
	}
	fmt.Println(total)
}

func getways(bags map[string][]Bag, ways map[string]int, b string) int {
	if v, ok := ways[b]; ok {
		return v
	}
	if bags[b] == nil {
		ways[b] = 0
	} else {
		for _, v := range bags[b] {
			ways[b] += getways(bags, ways, v.name)
		}
	}
	return ways[b]
}
