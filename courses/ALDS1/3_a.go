package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func calc(a, b int, op string) int {
	switch op {
	case "+":
		return a + b
	case "-":
		return a - b
	case "*":
		return a * b
	default:
		panic("unknown operator")
	}
}

func main() {
	reader := bufio.NewReader(os.Stdin)
	st := make([]int, 0, 101)
	line, _ := reader.ReadString('\n')
	line = strings.TrimSpace(line)
	for _, s := range strings.Split(line, " ") {
		if s[0] >= '0' && s[0] <= '9' {
			x, _ := strconv.Atoi(s)
			st = append(st, x)
		} else {
			a, b := st[len(st)-2], st[len(st)-1]
			st = append(st[:len(st)-2], calc(a, b, s))
		}
	}
	fmt.Println(st[len(st)-1])
}
