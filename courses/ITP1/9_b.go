package main

import "fmt"

func main() {
	for {
		var s string
		fmt.Scan(&s)
		if s == "-" {
			break
		}
		var m int
		fmt.Scan(&m)
		h := 0
		for i := 0; i < m; i++ {
			var x int
			fmt.Scan(&x)
			h += x
		}
		h %= len(s)
		fmt.Print(s[h:])
		fmt.Println(s[0:h])
	}
}
