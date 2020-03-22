package main

import (
	"fmt"
	"reflect"
)

func right(tp, fr int) int {
	r := [][]int{
		{},
		{2, 3, 5, 4},
		{6, 3, 1, 4},
		{2, 6, 5, 1},
	}
	n := 1
	if tp > 3 {
		tp = 7 - tp
		n = 3
	}
	for k := 0; k < 4; k++ {
		if r[tp][k] == fr {
			k += n
			k %= 4
			return r[tp][k]
		}
	}
	return 0
}

func ident(d1, d2 []int) bool {
	for i := 1; i <= 6; i++ {
		for j := 1; j <= 6; j++ {
			k := right(i, j)
			if k == 0 {
				continue
			}
			a := []int{d1[i], d1[j], d1[k], d1[7-k], d1[7-j], d1[7-i]}
			if reflect.DeepEqual(a, d2[1:]) {
				return true
			}
		}
	}
	return false
}

func combinations(n int) [][]int {
	ret := make([][]int, 0, n*(n-1)/2)
	for i := 0; i < n; i++ {
		for j := i + 1; j < n; j++ {
			ret = append(ret, []int{i, j})
		}
	}
	return ret
}

func main() {
	var n int
	fmt.Scan(&n)
	d := make([][]int, n)
	for i := 0; i < n; i++ {
		d[i] = make([]int, 7)
		for j := 1; j <= 6; j++ {
			fmt.Scan(&d[i][j])
		}
	}
	for _, c := range combinations(n) {
		if ident(d[c[0]], d[c[1]]) {
			fmt.Println("No")
			return
		}
	}
	fmt.Println("Yes")
}
