package main

import (
	"bufio"
	"fmt"
	"os"
)

func main() {
	reader := bufio.NewReader(os.Stdin)
	res := make(map[byte]int)

	for {
		s, err := reader.ReadString('\n')
		for i := 0; i < len(s); i++ {
			if s[i] < 'a' {
				res[s[i]+'a'-'A']++
			} else {
				res[s[i]]++
			}
		}
		if err != nil {
			break
		}
	}
	for i := byte('a'); i <= 'z'; i++ {
		fmt.Println(string(i), ":", res[i])
	}
}
