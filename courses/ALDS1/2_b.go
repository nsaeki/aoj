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
		mini := i
		for j := i; j < n; j++ {
			if a[j] < a[mini] {
				mini = j
			}
		}
		if i != mini {
			a[i], a[mini] = a[mini], a[i]
			cnt++
		}
	}
	fmt.Println(strings.Trim(fmt.Sprint(a), "[]"))
	fmt.Println(cnt)
}
