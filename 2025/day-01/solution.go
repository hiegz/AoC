package main

import (
	"fmt"
	"log"
	"math"
	"os"
	"strconv"
	"strings"
)

type direction int

const (
	left = iota
	right
)

type rotation struct {
	direction direction
	step      uint64
}

type state struct {
	position uint
	clicks   uint
}

func ParseInput(fileName string) []rotation {
	content, err := os.ReadFile(fileName)
	if err != nil {
		log.Fatal(err)
	}
	tokens := strings.Split(string(content), "\n")
	rotations := make([]rotation, len(tokens))

	for _, token := range tokens {
		if len(token) == 0 {
			continue
		}

		direction := func() direction {
			if token[0] == 'L' {
				return left
			}

			if token[0] == 'R' {
				return right
			}

			panic("invalid input")
		}()

		step, err := strconv.ParseUint(token[1:], 10, 64)
		if err != nil {
			panic("invalid input")
		}

		rotations = append(rotations, rotation{direction: direction, step: step})
	}

	return rotations
}

func PartOne(rotations []rotation) uint {
	counter := uint(0)
	state := state{position: uint(50)}
	for _, rotation := range rotations {
		switch rotation.direction {
		case left:
			state = RotateLeft(state.position, int(rotation.step))
		case right:
			state = RotateRight(state.position, int(rotation.step))
		}

		if state.position == 0 {
			counter += 1
		}
	}
	return counter
}

func PartTwo(rotations []rotation) uint {
	counter := uint(0)
	state := state{position: uint(50)}
	for _, rotation := range rotations {
		switch rotation.direction {
		case left:
			state = RotateLeft(state.position, int(rotation.step))
		case right:
			state = RotateRight(state.position, int(rotation.step))
		}

		counter += state.clicks
	}
	return counter
}

func RotateLeft(position uint, step int) state {
	return RotateRight(position, -step)
}

func RotateRight(position uint, step int) state {
	rangeSize := 100
	result := (int(position) + step)
	clicks := uint(math.Abs(math.Floor(float64(result / rangeSize))))
	result %= rangeSize
	if result < 0 {
		clicks += 1
		result += rangeSize
	}
	return state{position: uint(result), clicks: clicks}
}

func main() {
	testRotations := ParseInput("test-input.txt")
	rotations := ParseInput("input.txt")

	func() {
		fmt.Print("Part One ... ")
		expected := uint(3)
		found := PartOne(testRotations)
		if expected != found {
			fmt.Println("failed")
			os.Exit(0)
		}
		fmt.Println("OK")

		fmt.Print("Part One: ")
		solution := PartOne(rotations)
		fmt.Println(solution)
	}()

	func() {
		fmt.Print("Part Two ... ")
		expected := uint(6)
		found := PartTwo(testRotations)
		if expected != found {
			fmt.Printf("failed %d != %d\n", expected, found)
			os.Exit(0)
		}
		fmt.Println("OK")

		fmt.Print("Part Two: ")
		solution := PartTwo(rotations)
		fmt.Println(solution)
	}()
}
