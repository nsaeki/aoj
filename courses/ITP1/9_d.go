package main

import "fmt"

func main() {
	var q int
	var x string
	fmt.Scan(&x, &q)
	s := []byte(x)

	for i := 0; i < q; i++ {
		var cmd string
		var a, b int
		fmt.Scan(&cmd, &a, &b)
		switch cmd {
		case "print":
			fmt.Println(string(s[a : b+1]))
		case "reverse":
			for i := a; i <= (a+b)/2; i++ {
				s[i], s[b+a-i] = s[b+a-i], s[i]
			}
		case "replace":
			var p string
			fmt.Scan(&p)
			copy(s[a:b+1], []byte(p))
		}
	}
}
