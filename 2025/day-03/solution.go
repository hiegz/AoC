package main

import "math"
import "fmt"
import "os"
import "strings"

func Solve(testFileName string, batteries uint) uint {
	content, err := os.ReadFile(testFileName)
	if err != nil {
		panic("unable to read puzzle input")
	}

	lines := strings.Split(string(content), "\n")
	lines = lines[:len(lines)-1]

	sum := uint(0)

	for _, bank := range lines {
		bank := []byte(bank)
		selection := make([]int, batteries)
		prev := uint(0)
		selection[0] = -1

		for selected := range batteries {
			start := selection[prev] + 1
			end := len(bank) - int(batteries) + int(selected)
			selection[selected] = start

			for i, battery := range bank[start : end+1] {
				if battery > bank[selection[selected]] {
					selection[selected] = start + i
				}
			}

			prev = selected
		}

		for i, index := range selection {
			sum += uint(int(bank[index]-'0') * int(math.Pow10(int(batteries)-i-1)))
		}
	}

	return sum
}

func main() {
	func() {
		fmt.Print("Part One ... ")
		expected := uint(357)
		found := Solve("test-input.txt", 2)
		if expected != found {
			fmt.Printf("failed %d != %d\n", expected, found)
			os.Exit(0)
		}
		fmt.Println("OK")

		fmt.Print("Part One: ")
		solution := Solve("input.txt", 2)
		fmt.Println(solution)
	}()

	func() {
		fmt.Print("Part Two ... ")
		expected := uint(3121910778619)
		found := Solve("test-input.txt", 12)
		if expected != found {
			fmt.Printf("failed %d != %d\n", expected, found)
			os.Exit(0)
		}
		fmt.Println("OK")

		fmt.Print("Part Two: ")
		solution := Solve("input.txt", 12)
		fmt.Println(solution)
	}()
}
