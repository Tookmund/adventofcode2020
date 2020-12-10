package main
import (
	"bufio"
	"strings"
	"strconv"
	"fmt"
	"os"
)

func main () {
	scanner := bufio.NewScanner(os.Stdin)
	program := make([]string, 0)
	hasrun := make([]bool, 0)
	for scanner.Scan() {
		program = append(program, scanner.Text())
		hasrun = append(hasrun, false)
	}
	if scerr := scanner.Err(); scerr != nil {
		panic(scerr)
	}
	acc := 0
	pc := 0
	for {
		if hasrun[pc] {
			break
		}
		hasrun[pc] = true
		line := strings.Split(program[pc], " ")
		i, err := strconv.Atoi(line[1])
		if err != nil {
			panic(err)
		}
		switch line[0] {
		case "nop":
			// Nothing
		case "acc":
			acc += i
		case "jmp":
			pc += i
			continue
		}
		pc++
	}
	fmt.Println(acc)
}
