package main

import (
	"fmt"
)

func merge(a, b []int) []int {
	ret := []int{a[0] + b[0], a[1]}
	if ret[1] < b[1] {
		ret[1] = b[1]
	}
	return ret
}

func main() {
	var s string
	fmt.Scan(&s)
	areas := [][]int{}
	st := []int{}
	lv := 0

	for i := 0; i < len(s); i++ {
		switch s[i] {
		case '\\':
			lv--
			st = append(st, i)
		case '/':
			lv++
			if len(st) > 0 {
				x := st[len(st)-1]
				st = st[:len(st)-1]
				a := []int{i - x, lv}
				for j := len(areas) - 1; j >= 0; j-- {
					if areas[j][1] >= lv {
						break
					}
					a = merge(a, areas[j])
					areas = areas[:j]
				}
				areas = append(areas, a)
			}
			if len(areas) > 0 && areas[len(areas)-1][1] < lv {
				areas[len(areas)-1][1] = lv
			}
		}

	}

	sum := 0
	for i := 0; i < len(areas); i++ {
		sum += areas[i][0]
	}
	fmt.Println(sum)
	fmt.Print(len(areas))
	for i := 0; i < len(areas); i++ {
		fmt.Print(" ", areas[i][0])
	}
	fmt.Println()
}
