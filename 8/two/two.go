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

func solveTwo(maps map[string]Children, turns string) int {
	steps := 0

	for curr != "ZZZ" {
		turn := string(turns[steps % len(turns)])
		fmt.Printf("%d mod %d == %d so ", steps, len(turns), steps % len(turns))
		fmt.Printf("%s -%s-> ", curr, turn)
		switch turn{
		case "L":
			curr = maps[curr].l
		case "R":
			curr = maps[curr].r
		}
		steps++
		fmt.Printf("%s\n", curr)
	}
	fmt.Println("It took %d steps to reach %s", steps, curr)
	return steps
}

func main() {
	// Open the text file
	file, err := os.Open("./example.txt")
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
	starts := []string


	for _, line := range arr {
		maps[line[0:3]] = Children{line[7:10],line[12:15]}
		if line[2] == "Z" {
			append(starts, line[0:3])
		}
		//fmt.Printf("%s -> (%s, %s)\n", string(line[0:3]), string(line[7:10]), string(line[12:15]))
	}

	//fmt.Printf("%d", solveOne(maps, turns))
}