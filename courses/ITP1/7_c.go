package main

import "fmt"

func main() {
	var r, c int
	fmt.Scan(&r, &c)

	t := make([]int, c)
	for i := 0; i < r; i++ {
		s := 0
		for j := 0; j < c; j++ {
			var x int
			fmt.Scan(&x)
			fmt.Print(x, " ")
			s += x
			t[j] += x
		}
		fmt.Println(s)
	}
	s := 0
	for i := 0; i < c; i++ {
		fmt.Print(t[i], " ")
		s += t[i]
	}
	fmt.Println(s)
}
