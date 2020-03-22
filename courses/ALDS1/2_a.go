package main

import (
	"fmt"
	"strings"
)

func main() {
	var n int
	fmt.Scan(&n)
	a := make([]int, n)
	for i := 0; i < n; i++ {
		fmt.Scan(&a[i])
	}

	cnt := 0
	for i := 0; i < n; i++ {
		for j := n - 1; j > i; j-- {
			if a[j] < a[j-1] {
				a[j], a[j-1] = a[j-1], a[j]
				cnt++
			}
		}
	}
	fmt.Println(strings.Trim(fmt.Sprint(a), "[]"))
	fmt.Println(cnt)
}
