package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	reader := bufio.NewReader(os.Stdin)
	s, _ := reader.ReadString('\n')

	for i := 0; i < len(s); i++ {
		c := s[i]
		if c >= 'A' {
			if c < 'a' {
				fmt.Print(string(c + 'a' - 'A'))
			} else if c <= 'z' {
				fmt.Print(string(c - 'a' + 'A'))
			}
		} else {
			fmt.Print(string(c))
		}
	}
}
