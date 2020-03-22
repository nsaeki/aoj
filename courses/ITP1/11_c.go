package main

import (
	"fmt"
	"reflect"
)

func main() {
	d1 := make([]int, 7)
	d2 := make([]int, 7)
	for i := 1; i <= 6; i++ {
		var x int
		fmt.Scan(&x)
		d1[i] = x
	}
	for i := 1; i <= 6; i++ {
		var x int
		fmt.Scan(&x)
		d2[i] = x
	}

	/*
	   1: 2, 3, 5, 4
	   2: 6, 3, 1, 4
	   3: 2, 6, 5, 1
	*/
	r := [][]int{
		{},
		{2, 3, 5, 4},
		{6, 3, 1, 4},
		{2, 6, 5, 1},
	}
	rightFace := func(tp, fr int) int {
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

	for i := 1; i <= 6; i++ {
		for j := 1; j <= 6; j++ {
			k := rightFace(i, j)
			if k == 0 {
				continue
			}
			a := []int{d1[i], d1[j], d1[k], d1[7-k], d1[7-j], d1[7-i]}
			if reflect.DeepEqual(a, d2[1:]) {
				fmt.Println("Yes")
				return
			}
		}
	}
	fmt.Println("No")
}
