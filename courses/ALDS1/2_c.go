package main

import (
	"fmt"
	"reflect"
)

type card struct {
	s byte
	v int
}

func bubbleSort(a []*card) {
	n := len(a)
	for i := 0; i < n; i++ {
		for j := n - 1; j > i; j-- {
			if a[j].v < a[j-1].v {
				a[j], a[j-1] = a[j-1], a[j]
			}
		}
	}
}

func selectionSort(a []*card) {
	n := len(a)
	for i := 0; i < n; i++ {
		mini := i
		for j := i; j < n; j++ {
			if a[j].v < a[mini].v {
				mini = j
			}
		}
		if i != mini {
			a[i], a[mini] = a[mini], a[i]
		}
	}
}

func printCards(a []*card) {
	for i := 0; i < len(a); i++ {
		fmt.Printf("%c%d", a[i].s, a[i].v)
		if i == len(a)-1 {
			fmt.Println()
		} else {
			fmt.Print(" ")
		}
	}
}
func main() {
	var n int
	fmt.Scan(&n)
	c := make([]*card, n)
	for i := 0; i < n; i++ {
		var s string
		fmt.Scan(&s)
		c[i] = &card{s[0], int(s[1] - '0')}
	}

	bs := make([]*card, n)
	ss := make([]*card, n)
	copy(bs, c)
	copy(ss, c)
	bubbleSort(bs)
	selectionSort(ss)

	printCards(bs)
	fmt.Println("Stable")
	printCards(ss)
	if reflect.DeepEqual(bs, ss) {
		fmt.Println("Stable")
	} else {
		fmt.Println("Not stable")
	}
}
