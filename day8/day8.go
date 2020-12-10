package main

import (
	"bufio"
	"errors"
	"fmt"
	"os"
	"strconv"
	"strings"
)

type program_t [][]string

func main() {
	scanner := bufio.NewScanner(os.Stdin)
	program := make(program_t, 0)
	for scanner.Scan() {
		program = append(program, strings.Split(scanner.Text(), " "))
	}
	if scerr := scanner.Err(); scerr != nil {
		panic(scerr)
	}
	for i := range program {
		if program[i][0] == "nop" {
			program[i][0] = "jmp"
			if acc, err := runProgram(program); err == nil {
				fmt.Println("Changed", i+1, "to JMP")
				fmt.Println("ACC:", acc)
				break
			}
			program[i][0] = "nop"
		} else if program[i][0] == "jmp" {
			program[i][0] = "nop"
			if acc, err := runProgram(program); err == nil {
				fmt.Println("Changed", i+1, "to NOP")
				fmt.Println("ACC:", acc)
				break
			}
			program[i][0] = "jmp"
		}
	}
}

func runProgram(program program_t) (int, error) {
	hasrun := make([]bool, len(program))
	acc := 0
	pc := 0
	reterr := errors.New("Does Not Terminate")
	for {
		if pc == len(program) {
			fmt.Println("SUCCESS")
			reterr = nil
			break
		}
		fmt.Println(pc, program[pc])
		if hasrun[pc] {
			break
		}
		hasrun[pc] = true
		i, err := strconv.Atoi(program[pc][1])
		if err != nil {
			panic(err)
		}
		switch program[pc][0] {
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
	return acc, reterr
}
