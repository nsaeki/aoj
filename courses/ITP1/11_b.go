package main

import "fmt"

func memo(m [][]int, r, t, f int) {
	arr := []int{t, f, 7 - t, 7 - f, t}
	for i := 0; i < len(arr)-1; i++ {
		ti, fi := arr[i], arr[i+1]
		m[ti][fi] = r
	}
}

func main() {
	d := make([]int, 7)
	for i := 1; i < 7; i++ {
		var x int
		fmt.Scan(&x)
		d[i] = x
	}

	m := make([][]int, 7)
	for i := 0; i < 7; i++ {
		m[i] = make([]int, 7)
	}
	memo(m, 1, 4, 2)
	memo(m, 2, 1, 4)
	memo(m, 3, 1, 2)
	memo(m, 4, 2, 1)
	memo(m, 5, 4, 1)
	memo(m, 6, 2, 4)

	var q int
	fmt.Scan(&q)
	for i := 0; i < q; i++ {
		var t, f int
		fmt.Scan(&t, &f)
		var ti, fi int
		for i := 0; i < len(d); i++ {
			if t == d[i] {
				ti = i
			}
			if f == d[i] {
				fi = i
			}
		}
		fmt.Println(d[m[ti][fi]])
	}
}
