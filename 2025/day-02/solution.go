package main

import (
	"fmt"
	"math"
	"os"
	"strconv"
	"strings"
)

type Range struct {
	start uint64
	end   uint64
}

func ParseInput(fileName string) []Range {
	contents, err := os.ReadFile(fileName)
	if err != nil {
		panic("unable to read the input file")
	}

	tokens := strings.Split(string(contents), ",")
	ranges := make([]Range, len(tokens))

	for _, token := range tokens {
		token := strings.TrimSpace(token)
		rg := strings.SplitN(token, "-", 2)

		start, err := strconv.ParseUint(rg[0], 10, 64)
		if err != nil {
			panic("invalid input")
		}

		end, err := strconv.ParseUint(rg[1], 10, 64)
		if err != nil {
			panic("invalid input")
		}

		ranges = append(ranges, Range{start, end})
	}

	return ranges
}

func PartOne(ranges []Range) uint64 {
	sum := uint64(0)

	for _, rg := range ranges {
		for i := rg.start; i <= rg.end; i++ {
			number := strconv.FormatUint(i, 10)
			length := len(number)

			// skip odd-length numbers
			if length%2 != 0 {
				i = uint64(math.Pow10(len(number))) - 1
			}

			mid := length / 2

			if number[:mid] == number[mid:] {
				sum += i
			}
		}
	}

	return sum
}

func PartTwo(ranges []Range) uint64 {
	sum := uint64(0)

	for _, rg := range ranges {
		for i := rg.start; i <= rg.end; i++ {
			number := strconv.FormatUint(i, 10)
			length := len(number)

		Outer:
			for buckets := 2; buckets <= length; buckets++ {
				if length%buckets != 0 {
					continue
				}

				step := length / buckets

				for start := step; start < length; start += step {
					if number[start-step:start] != number[start:start+step] {
						continue Outer
					}
				}

				sum += i
				break
			}
		}
	}

	return sum
}

func main() {
	testRanges := ParseInput("test-input.txt")
	ranges := ParseInput("input.txt")

	func() {
		fmt.Print("Part One ... ")
		expected := uint64(1227775554)
		found := PartOne(testRanges)
		if expected != found {
			fmt.Printf("failed %d != %d\n", expected, found)
			os.Exit(0)
		}
		fmt.Println("OK")

		fmt.Print("Part One: ")
		solution := PartOne(ranges)
		fmt.Println(solution)
	}()

	func() {
		fmt.Print("Part Two ... ")
		expected := uint64(4174379265)
		found := PartTwo(testRanges)
		if expected != found {
			fmt.Printf("failed %d != %d\n", expected, found)
			os.Exit(0)
		}
		fmt.Println("OK")

		fmt.Print("Part Two: ")
		solution := PartTwo(ranges)
		fmt.Println(solution)
	}()
}
