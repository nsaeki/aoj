package main

import "fmt"

func main() {
	var a, b int
	fmt.Scan(&a, &b)

	if a > b {
		a, b = b, a
	}

	for b%a != 0 {
		a, b = b%a, a
	}
	fmt.Println(a)
}
