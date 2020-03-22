package main

import "fmt"

func main() {
	for {
		var m, f, r int
		fmt.Scan(&m, &f, &r)
		if m == -1 && f == -1 && r == -1 {
			break
		}
		t := m + f
		g := ""
		if m == -1 || f == -1 || t < 30 {
			g = "F"
		} else if t >= 80 {
			g = "A"
		} else if t >= 65 {
			g = "B"
		} else if t >= 50 {
			g = "C"
		} else if t >= 30 {
			if r >= 50 {
				g = "C"
			} else {
				g = "D"
			}
		}
		fmt.Println(g)
	}
}
