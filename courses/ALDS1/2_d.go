package main

import (
	"bufio"
	"fmt"
	"os"
	"strconv"
	"strings"
)

func newScanner() *bufio.Scanner {
	scanner := bufio.NewScanner(os.Stdin)
	scanner.Split(bufio.ScanWords)
	return scanner
}

func scanInt() int {
	sc.Scan()
	v, _ := strconv.Atoi(sc.Text())
	return v
}

func scanInts(n int) []int {
	a := make([]int, n)
	for i := 0; i < n; i++ {
		a[i] = scanInt()
	}
	return a
}

var sc = newScanner()

func insertionSort(a []int, g int) int {
	n := len(a)
	cnt := 0
	for i := g; i < n; i++ {
		v := a[i]
		j := i - g
		for j >= 0 && a[j] > v {
			a[j+g] = a[j]
			j -= g
			cnt++
		}
		a[j+g] = v
	}
	return cnt
}

func main() {
	n := scanInt()
	a := scanInts(n)
	cnt := 0
	g := []int{}
	for _, i := range []int{701, 301, 132, 57, 23, 10, 4, 1} {
		if n > i {
			g = append(g, i)
		}
	}
	if len(g) == 0 {
		g = append(g, 1)
	}
	m := len(g)
	for i := 0; i < m; i++ {
		cnt += insertionSort(a, g[i])
	}

	fmt.Println(m)
	fmt.Println(strings.Trim(fmt.Sprint(g), "[]"))
	fmt.Println(cnt)
	for i := 0; i < n; i++ {
		fmt.Println(a[i])
	}
}
