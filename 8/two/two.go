package main

import (
	"strings"
	"fmt"
	"os"
	"io"
)

type Children struct {
	l string
	r string
}

func havelengths(lengths []int) bool {
	//fmt.Printf("%#v", curr)
	for _, l := range lengths {
		if l == 0 {
			return false
		}
		//fmt.Printf(node)
	}

	return true
}

func GCD(a, b int) int {
	for b != 0 {
			t := b
			b = a % b
			a = t
	}
	return a
}

// find Least Common Multiple (LCM) via GCD
func LCM(a, b int, integers ...int) int {
	result := a * b / GCD(a, b)

	for i := 0; i < len(integers); i++ {
			result = LCM(result, integers[i])
	}

	return result
}

func solveTwo(maps map[string]Children, starts []string, turns string) int {
	steps := 0
	currs := make([]string, len(starts))
	lengths := make([]int, len(starts))
	copy(currs, starts)

	//fmt.Printf("%#v", lengths, starts)

	for !havelengths(lengths) {
		turn := string(turns[steps % len(turns)])
		//fmt.Printf("%d mod %d == %d so\n", steps, len(turns), steps % len(turns))
		for i,curr := range currs {
			//fmt.Printf("%s -%s-> ", curr, turn)
			if string(curr[2]) == "Z" {
				lengths[i] = steps;
			}
			switch turn{
			case "L":
				currs[i] = maps[curr].l
			case "R":
				currs[i] = maps[curr].r
		}	}
			//fmt.Printf("%s\n", curr)

		steps++
	}
	//fmt.Println("It took %d steps to reach %s", steps, curr)
	return LCM(lengths[0], lengths[1], lengths[:len(lengths)]...)
}

func main() {
	// Open the text file
	file, err := os.Open("./input.txt")
	if err != nil {
		fmt.Println("Error opening file:", err)
		return
	}
	defer file.Close()

	var buf strings.Builder
	io.Copy(&buf, file)
	s := buf.String()
	arr := strings.Split(s, "\r\n")
	turns := arr[0]
	arr = arr[2:len(arr)]
	maps := make(map[string]Children)
	starts := []string{}


	for _, line := range arr {
		maps[line[0:3]] = Children{line[7:10],line[12:15]}
		if string(line[2]) == "A" {
			starts = append(starts, string(line[0:3]))
		}
		//fmt.Printf("%s -> (%s, %s)\n", string(line[0:3]), string(line[7:10]), string(line[12:15]))
	}
	//fmt.Printf("%#v", starts[0])
	fmt.Printf("%d", solveTwo(maps, starts, turns))
}