package main

import "fmt"

func main() {
	var n int
	fmt.Scan(&n)
	var t, h int
	for i := 0; i < n; i++ {
		var c1, c2 string
		fmt.Scan(&c1, &c2)
		if c1 > c2 {
			t += 3
		} else if c1 < c2 {
			h += 3
		} else {
			t++
			h++
		}
	}
	fmt.Println(t, h)
}
