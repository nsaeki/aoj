package main

import "fmt"

func rot(d []int, a ...int) {
	d[a[0]], d[a[1]], d[a[2]], d[a[3]] = d[a[1]], d[a[2]], d[a[3]], d[a[0]]
}

func main() {
	d := make([]int, 7)
	for i := 1; i <= 6; i++ {
		var x int
		fmt.Scan(&x)
		d[i] = x
	}
	var s string
	fmt.Scan(&s)
	for i := 0; i < len(s); i++ {
		switch s[i] {
		case 'N':
			rot(d, 1, 2, 6, 5)
		case 'E':
			rot(d, 1, 4, 6, 3)
		case 'W':
			rot(d, 1, 3, 6, 4)
		case 'S':
			rot(d, 1, 5, 6, 2)
		}
	}
	fmt.Println(d[1])
}
