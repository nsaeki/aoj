package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func newScanner() *bufio.Scanner {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanWords)
	return scanner
}

func scanString() string {
	if sc.Scan() {
		return sc.Text()
	}
	panic(sc.Err())
}

var sc = newScanner()

func main() {
	w := strings.ToLower(scanString())
	ans := 0
	for {
		t := scanString()
		if t == "END_OF_TEXT" {
			break
		}
		if w == strings.ToLower(t) {
			ans++
		}
	}
	fmt.Println(ans)
}
