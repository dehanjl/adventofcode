package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
)

func part1(numbers []int) {
	increaseCount := 0
	for i := 1; i < len(numbers); i++ {
		if numbers[i] > numbers[i-1] {
			increaseCount++
		}
	}

	fmt.Println(increaseCount)
}

func part2(numbers []int) {
	windowSums := make([]int, 0)
	for i := 2; i < len(numbers); i++ {
		sum := numbers[i] + numbers[i-1] + numbers[i-2]
		windowSums = append(windowSums, sum)
	}

	part1(windowSums)
}

func main() {
	// File reading boilerplate
	readFile, err := os.Open("input.txt")

	if err != nil {
		fmt.Println(err)
	}
	fileScanner := bufio.NewScanner(readFile)

	fileScanner.Split(bufio.ScanLines)

	numbers := make([]int, 0)

	for fileScanner.Scan() {
		val, _ := strconv.Atoi(fileScanner.Text())
		numbers = append(numbers, val)
	}

	part1(numbers)
	part2(numbers)

	readFile.Close()

}
