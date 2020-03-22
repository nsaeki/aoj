package main

import "fmt"

func main() {
	var n, m, l int
	fmt.Scan(&n, &m, &l)
	a := make([][]int, n)
	b := make([][]int, m)
	for i := 0; i < n; i++ {
		a[i] = make([]int, m)
		for j := 0; j < m; j++ {
			fmt.Scan(&a[i][j])
		}
	}

	for i := 0; i < m; i++ {
		b[i] = make([]int, l)
		for j := 0; j < l; j++ {
			fmt.Scan(&b[i][j])
		}
	}

	for i := 0; i < n; i++ {
		for j := 0; j < l; j++ {
			c := 0
			for k := 0; k < m; k++ {
				c += a[i][k] * b[k][j]
			}
			fmt.Print(c)
			if j < l-1 {
				fmt.Print(" ")
			} else {
				fmt.Println()
			}
		}
	}
}
